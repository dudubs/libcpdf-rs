use super::core::*;
use super::document::*;
use super::range::*;

const TESTDATA_1PAGES: &str = "testdata/1pages.pdf";
const TESTDATA_2PAGES: &str = "testdata/2pages.pdf";

#[test]
fn test_range() -> Result {
    startup()?;

    assert_eq!(Range::only(1)?.has(1)?, true);
    assert_eq!(Range::only(1)?.has(2)?, false);
    assert_eq!(Range::only(1)?.merge(Range::only(2)?)?.has(2)?, true);
    assert_eq!(Range::only(1)?.merge(Range::only(3)?)?.has(2)?, false);
    Ok(())
}

#[test]
fn test_file_from_file() -> Result {
    startup()?;

    assert_eq!(Document::from_file(TESTDATA_1PAGES, "")?.num_pages()?, 1);
    assert_eq!(Document::from_file(TESTDATA_2PAGES, "")?.num_pages()?, 2);

    Ok(())
}

#[test]
fn test_from_mem() -> Result {
    startup()?;

    assert_eq!(
        Document::from_mem(std::fs::read(TESTDATA_1PAGES)?, "")?.num_pages()?,
        1
    );
    assert_eq!(
        Document::from_mem(std::fs::read(TESTDATA_2PAGES)?, "")?.num_pages()?,
        2
    );

    Ok(())
}

#[test]
fn test_get_media_box() -> Result {
    startup()?;

    let doc = Document::from_file("testdata/unfit.pdf", "")?;
    assert_eq!(
        doc.get_media_box(1)?,
        Box {
            min_x: 0.0,
            min_y: 0.0,
            max_x: 595.276,
            max_y: 841.89,
        }
    );
    assert_eq!(
        doc.get_media_box(2)?,
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
fn test_fit_to_with() -> Result {
    startup()?;
    let doc = Document::from_file("testdata/unfit.pdf", "")?;

    doc.fit_to_width(200.0, 0.0)?;

    let doc = Document::from_mem(doc.to_vec()?, "")?;
    assert_eq!(doc.get_media_box(1)?.width(), 200.0);
    assert_eq!(doc.get_media_box(2)?.width(), 200.0);

    doc.fit_to_width(205.0, 6.0)?;
    let doc = Document::from_mem(doc.to_vec()?, "")?;
    assert_eq!(doc.get_media_box(1)?.width(), 200.0);
    assert_eq!(doc.get_media_box(2)?.width(), 200.0);

    doc.fit_to_width(205.0, 2.0)?;
    let doc = Document::from_mem(doc.to_vec()?, "")?;
    assert_eq!(doc.get_media_box(1)?.width(), 205.0);
    assert_eq!(doc.get_media_box(2)?.width(), 205.0);

    Ok(())
}

#[test]
fn dev() -> Result {
    startup()?;

    let doc = Document::from_mem(std::fs::read("../../testdata/big.pdf").unwrap(), "")?;
    let doc = doc.select_pages(Range::between(2, 72)?)?;
    doc.to_vec()?;

    Ok(())
}
