use document::Document;
use range::Range;

pub mod bindings;
pub mod core;
pub mod document;
pub mod error;
pub mod range;
mod tests;

// cpdfrouter:load() cpdfrouter::save({close:true})
#[test]
fn dev() {
    core::startup();

    core::set_fast();

    // // unsafe { dbg!(cpdf_lastError) };
    // dbg!(cpdf::version().unwrap());
    // let mut file = cpdf::File::from_mem(std::fs::read("test.pdf").unwrap(), "").unwrap();

    // // dbg!(file.get_media_box(1));
    // // dbg!(file.num_pages());
    // // fit_to_width(&file, 200.0).unwrap();

    // fit_to_width(&file, 200.0).unwrap();
    // unsafe {
    //     let r = cpdf::Range::only(2).unwrap();
    //     bindings::cpdf_rotateBy(file.id, r.id, 90);
    // }

    // file.save_as("test-out.pdf").unwrap();
    // println!("Hello, world!");
}

pub fn fit_to_width(file: &Document, width: f64) -> core::Result<()> {
    for page_num in 1..file.num_pages()? + 1 {
        let media_box = file.get_media_box(page_num)?;
        let scale = width / media_box.width();
        file.scale_pages(Range::only(page_num)?, scale, scale)?;
    }

    Ok(())
}
