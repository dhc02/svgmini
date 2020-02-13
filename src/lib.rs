use svgcleaner::{self, CleaningOptions};
use svgdom::WriteOptions;
pub mod defaults;

pub fn minify_svg(svg_text: &str) -> String {
    let mut doc = svgdom::Document::from_str(svg_text).unwrap();
    let _ = svgcleaner::cleaner::clean_doc(&mut doc, &cleaning_options(), &write_options());
    doc.to_string().trim().to_string()
}

fn cleaning_options() -> CleaningOptions {
    CleaningOptions {
        remove_unused_defs: true,
        convert_shapes: true,
        remove_title: true,
        remove_desc: true,
        remove_metadata: true,
        remove_dupl_linear_gradients: true,
        remove_dupl_radial_gradients: true,
        remove_dupl_fe_gaussian_blur: true,
        ungroup_groups: true,
        ungroup_defs: true,
        group_by_style: true,
        merge_gradients: true,
        regroup_gradient_stops: true,
        remove_invalid_stops: true,
        remove_invisible_elements: true,
        resolve_use: true,

        remove_version: true,
        remove_unreferenced_ids: true,
        trim_ids: true,
        remove_text_attributes: true,
        remove_unused_coordinates: true,
        remove_default_attributes: true,
        remove_xmlns_xlink_attribute: true,
        remove_needless_attributes: true,
        remove_gradient_attributes: true,
        apply_transform_to_gradients: true,
        apply_transform_to_shapes: true,
        paths_to_relative: true,
        remove_unused_segments: true,
        convert_segments: true,
        apply_transform_to_paths: false,
        coordinates_precision: 6,
        properties_precision: 6,
        paths_coordinates_precision: 8,
        transforms_precision: 8,
        ..Default::default()
    }
}

fn write_options() -> WriteOptions {
    WriteOptions {
        use_single_quote: false,
        ..Default::default()
    }
}
