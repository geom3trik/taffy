pub fn compute() {
    let mut taffy = taffy::Taffy::new();
    let node0 = taffy
        .new_with_children(
            taffy::style::Style {
                flex_grow: 1f32,
                min_size: taffy::geometry::Size { width: taffy::style::Dimension::Points(60f32), ..Default::default() },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node1 = taffy
        .new_with_children(
            taffy::style::Style {
                flex_grow: 1f32,
                min_size: taffy::geometry::Size { width: taffy::style::Dimension::Points(60f32), ..Default::default() },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node2 = taffy
        .new_with_children(
            taffy::style::Style {
                flex_grow: 1f32,
                min_size: taffy::geometry::Size { width: taffy::style::Dimension::Points(60f32), ..Default::default() },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node3 = taffy
        .new_with_children(
            taffy::style::Style {
                flex_grow: 1f32,
                min_size: taffy::geometry::Size { width: taffy::style::Dimension::Points(60f32), ..Default::default() },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node4 = taffy
        .new_with_children(
            taffy::style::Style {
                flex_grow: 1f32,
                min_size: taffy::geometry::Size { width: taffy::style::Dimension::Points(60f32), ..Default::default() },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                flex_wrap: taffy::style::FlexWrap::Wrap,
                gap: taffy::geometry::Size { width: taffy::style::Dimension::Points(5f32), ..Default::default() },
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Points(300f32),
                    height: taffy::style::Dimension::Points(300f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[node0, node1, node2, node3, node4],
        )
        .unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::MAX_CONTENT).unwrap();
}
