#[test]
fn rounding_flex_basis_flex_shrink_row() {
    let mut taffy = taffy::Taffy::new();
    let node0 = taffy
        .new_with_children(
            taffy::style::Style {
                flex_shrink: 1f32,
                flex_basis: taffy::style::Dimension::Points(100f32),
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node1 = taffy
        .new_with_children(
            taffy::style::Style { flex_basis: taffy::style::Dimension::Points(25f32), ..Default::default() },
            &[],
        )
        .unwrap();
    let node2 = taffy
        .new_with_children(
            taffy::style::Style { flex_basis: taffy::style::Dimension::Points(25f32), ..Default::default() },
            &[],
        )
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Points(101f32),
                    height: taffy::style::Dimension::Points(100f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[node0, node1, node2],
        )
        .unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::MAX_CONTENT).unwrap();
    assert_eq!(taffy.layout(node).unwrap().size.width, 101f32);
    assert_eq!(taffy.layout(node).unwrap().size.height, 100f32);
    assert_eq!(taffy.layout(node).unwrap().location.x, 0f32);
    assert_eq!(taffy.layout(node).unwrap().location.y, 0f32);
    assert_eq!(taffy.layout(node0).unwrap().size.width, 67f32);
    assert_eq!(taffy.layout(node0).unwrap().size.height, 100f32);
    assert_eq!(taffy.layout(node0).unwrap().location.x, 0f32);
    assert_eq!(taffy.layout(node0).unwrap().location.y, 0f32);
    assert_eq!(taffy.layout(node1).unwrap().size.width, 17f32);
    assert_eq!(taffy.layout(node1).unwrap().size.height, 100f32);
    assert_eq!(taffy.layout(node1).unwrap().location.x, 67f32);
    assert_eq!(taffy.layout(node1).unwrap().location.y, 0f32);
    assert_eq!(taffy.layout(node2).unwrap().size.width, 17f32);
    assert_eq!(taffy.layout(node2).unwrap().size.height, 100f32);
    assert_eq!(taffy.layout(node2).unwrap().location.x, 84f32);
    assert_eq!(taffy.layout(node2).unwrap().location.y, 0f32);
}
