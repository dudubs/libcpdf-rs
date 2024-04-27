#![cfg(test)]

use std::thread;

use super::core::*;
use super::document::*;
use super::range::*;
use crate::bindings::CpdfPosition;
use crate::bindings::CPDF_ANCHOR_DIAGONAL;
use crate::bindings::CPDF_FONT_COURIER;

#[test]
fn test_range() -> Result {
    startup()?;

    assert_eq!(Range::only(1)?.has(1)?, true);
    assert_eq!(Range::only(1)?.has(2)?, false);
    assert_eq!(Range::only(1)?.merge(Range::only(2)?)?.has(2)?, true);
    assert_eq!(Range::only(1)?.merge(Range::only(3)?)?.has(2)?, false);

    assert_eq!(Range::from(&vec![1, 3])?.has(1)?, true);
    assert_eq!(Range::from(&vec![1, 3])?.has(2)?, false);
    assert_eq!(Range::from(&vec![1, 3])?.has(3)?, true);
    Ok(())
}

#[test]
fn test_multithreads() {
    let _ = (1..300)
        .map(|i| {
            thread::spawn(move || -> super::core::Result<Document> {
                startup()?;

                // let d1 = Document::from_file("testdata/3pages.pdf", "").unwrap();
                let d = Document::blank(1, 10.0, 10.0).unwrap();
                d.rotate_pages(&Range::only(1)?, 1)?;
                // thread::sleep(Duration::from_secs(3));

                Ok(d)
            })
        })
        .collect::<Vec<_>>()
        .into_iter()
        .map(|jh| jh.join().unwrap())
        .collect::<Vec<_>>();
}

#[test]
fn test_file_from_file() -> Result {
    startup()?;

    assert_eq!(
        Document::from_file("testdata/1pages.pdf", "")?.num_pages()?,
        1
    );
    assert_eq!(
        Document::from_file("testdata/2pages.pdf", "")?.num_pages()?,
        2
    );

    Ok(())
}

#[test]
fn test_from_mem() -> Result {
    startup()?;

    assert_eq!(
        Document::from_mem(std::fs::read("testdata/1pages.pdf")?, "")?.num_pages()?,
        1
    );
    assert_eq!(
        Document::from_mem(std::fs::read("testdata/2pages.pdf")?, "")?.num_pages()?,
        2
    );

    Ok(())
}

#[test]
fn test_media_box() -> Result {
    startup()?;

    let doc = Document::from_file("testdata/unfit.pdf", "")?;
    assert_eq!(
        doc.media_box(1)?,
        Box {
            min_x: 0.0,
            min_y: 0.0,
            max_x: 595.276,
            max_y: 841.89,
        }
    );
    assert_eq!(
        doc.media_box(2)?,
        Box {
            min_x: 0.0,
            min_y: 0.0,
            max_x: 297.638,
            max_y: 420.945
        }
    );

    Ok(())
}

#[test]
fn test_fit_to_width() -> Result {
    startup()?;
    let doc = Document::from_file("testdata/unfit.pdf", "")?;

    doc.fit_to_width(200.0, 0.0)?;

    let doc = Document::from_mem(doc.to_vec()?, "")?;
    assert_eq!(doc.media_box(1)?.width(), 200.0);
    assert_eq!(doc.media_box(2)?.width(), 200.0);

    doc.fit_to_width(205.0, 6.0)?;
    let doc = Document::from_mem(doc.to_vec()?, "")?;
    assert_eq!(doc.media_box(1)?.width(), 200.0);
    assert_eq!(doc.media_box(2)?.width(), 200.0);

    doc.fit_to_width(205.0, 2.0)?;
    let doc = Document::from_mem(doc.to_vec()?, "")?;
    assert_eq!(doc.media_box(1)?.width(), 205.0);
    assert_eq!(doc.media_box(2)?.width(), 205.0);

    Ok(())
}

#[test]
fn test_select_pages() -> Result {
    startup()?;
    let doc = Document::from_file("testdata/unfit.pdf", "")?;

    assert_eq!(doc.select_pages(&Range::only(1)?)?.num_pages()?, 1);
    Ok(())
}

#[test]
fn test_expect_to_rotate_and_fit() -> Result {
    startup()?;
    let doc = Document::from_file("testdata/2pages.pdf", "")?;

    assert_ne!(doc.page_size(1)?.0, 200.0);
    assert_ne!(doc.page_size(2)?.0, 200.0);

    // first fit to 200
    doc.fit_to_width(200.0, 0.0)?;
    assert_eq!(doc.page_size(1)?.0, 200.0);
    assert_eq!(doc.page_size(2)?.0, 200.0);
    assert_ne!(doc.page_size(1)?.0, doc.media_box(1)?.height());
    assert_ne!(doc.page_size(2)?.0, doc.media_box(2)?.height());

    doc.rotate_pages(&Range::only(2)?, 1)?;
    assert_eq!(doc.page_rotation(1)?, 0);
    assert_eq!(doc.page_rotation(2)?, 1);
    assert_eq!(doc.page_size(1)?.0, 200.0);
    assert_eq!(doc.page_size(2)?.1, 200.0);
    assert_ne!(doc.page_size(2)?.0, 200.0);

    doc.fit_to_width(200.0, 0.0)?;
    assert_eq!(doc.page_rotation(1)?, 0);
    assert_eq!(doc.page_rotation(2)?, 1);
    assert_eq!(doc.page_size(1)?.0, 200.0);
    assert_eq!(doc.page_size(2)?.0, 200.0); // *diff
    assert_ne!(doc.page_size(2)?.1, 200.0);

    Ok(())
}

#[test]
fn test_debug_fit_to_width() -> Result {
    startup()?;

    let mut vd = vec![];

    for r in [
        0,   //
        90,  //
        180, //
        270, //
    ] {
        for (w, h) in [
            (100.0, 300.0), // h>w
            (200.0, 300.0), // h>w
            (300.0, 300.0), // h>w
            (400.0, 200.0), // w>h
            (500.0, 200.0), // w>h
        ] {
            let d = Document::blank(1, w, h)?;
            let x = if w > h { "w>h" } else { "h>w" };
            d.rotate_pages(&Range::only(1)?, r / 90)?;
            d.add_test_text(1, format!("{w}x{h} {r}° {x}"))?;
            vd.push(d);
        }
    }

    let d = Document::merge(&vd, false)?;

    let fw = 100.0;
    d.fit_to_width(fw, 0.0)?;

    for page_num in 1..d.num_pages()? + 1 {
        let (mw, mh) = d.media_size(page_num)?;
        let (pw, _ph) = d.page_size(page_num)?;
        assert_eq!(pw, fw);
        dbg!((mw, mh));
    }

    // d.save_as("testdata/_debug.pdf")?;

    Ok(())
}

#[test]
fn test_page_size() -> Result {
    startup()?;
    let doc = Document::from_file("testdata/3pages.pdf", "")?;

    let media_box = doc.media_box(1)?;
    let mw = media_box.width();
    let mh = media_box.height();
    assert_ne!(mw, mh);

    assert_eq!(doc.page_rotation(1)?, 0);
    assert_eq!(doc.page_size(1)?, (mw, mh));
    assert_eq!(doc.media_size(1)?, (mw, mh));

    doc.rotate_pages(&Range::only(1)?, 1)?;
    assert_eq!(doc.page_rotation(1)?, 1);
    assert_eq!(doc.page_size(1)?, (mh, mw));
    assert_eq!(doc.media_size(1)?, (mw, mh));

    doc.rotate_pages(&Range::only(1)?, 1)?;
    assert_eq!(doc.page_rotation(1)?, 2);
    assert_eq!(doc.page_size(1)?, (mw, mh));
    assert_eq!(doc.media_size(1)?, (mw, mh));

    doc.rotate_pages(&Range::only(1)?, 1)?;
    assert_eq!(doc.page_rotation(1)?, 3);
    assert_eq!(doc.page_size(1)?, (mh, mw));
    assert_eq!(doc.media_size(1)?, (mw, mh));

    doc.rotate_pages(&Range::only(1)?, 1)?;
    assert_eq!(doc.page_rotation(1)?, 0);
    assert_eq!(doc.page_size(1)?, (mw, mh));
    assert_eq!(doc.media_size(1)?, (mw, mh));

    Ok(())
}

impl Document {
    fn add_test_text(&self, page_num: i32, text: impl ToChars) -> Result {
        self.add_text_simple(
            &Range::only(page_num)?,
            text,
            CpdfPosition {
                cpdf_anchor: CPDF_ANCHOR_DIAGONAL,
                cpdf_coord1: 0.0,
                cpdf_coord2: 0.0,
            },
            CPDF_FONT_COURIER,
            12.0,
        )
    }
}
