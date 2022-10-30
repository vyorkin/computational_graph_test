use computational_graph_test::dsl::*;

#[test]
fn sum_lit_lit() {
    let l1 = lit(5f32);
    let l2 = lit(37f32);

    let graph = add(l1, l2);
    let result = graph.compute();

    assert_eq!(result, Some(42f32));
}

#[test]
fn sum_var_lit() {
    let l1 = lit(5f32);
    let v1 = var("v1");
    let graph = add(v1.clone().into(), l1);

    v1.set(5f32);

    let result = graph.compute();

    assert_eq!(result, Some(10f32));
}

#[test]
fn sum_var_unset_lit() {
    let l1 = lit(5f32);
    let v1 = var("v1");

    let graph = add(v1.into(), l1);
    let result = graph.compute();

    assert_eq!(result, None);
}

#[test]
fn sum_var_var() {
    let v1 = var("v1");
    let v2 = var("v2");

    let graph = add(v1.clone().into(), v2.clone().into());

    v1.set(2f32);
    v2.set(3f32);

    assert_eq!(graph.compute(), Some(5f32));

    v1.set(6f32);
    v2.set(9f32);

    assert_eq!(graph.compute(), Some(15f32));
}

#[test]
fn product_var_var_clear() {
    let x1 = var("x1");
    let x2 = var("x2");

    x1.set(20f32);
    x2.set(30f32);

    let graph = mul(x1.clone().into(), x2.clone().into());

    assert_eq!(graph.compute(), Some(600f32));

    x1.set(50f32);
    x2.clear();

    assert_eq!(graph.compute(), None);
}

#[test]
fn polynomial() {
    let x = var("x");
    let y = var("y");

    let g = add(
        add(
            pow(x.clone().into(), lit(3f32)),
            mul(lit(2f32), y.clone().into()),
        ),
        lit(12f32),
    );

    x.set(2f32);
    y.set(4f32);

    // 2^3 + 2*4 + 12 = 28
    // ^       ^
    // x       y

    assert_eq!(g.compute(), Some(28f32));
}

#[test]
fn complex() {
    let x1 = var("x1");
    let x2 = var("x2");
    let x3 = var("x3");

    // x1 + x2 * sin(x2 + pow(x3, 3))

    let g = add(
        x1.clone().into(),
        mul(
            x2.clone().into(),
            sin(add(x2.clone().into(), pow(x3.clone().into(), lit(3f32)))),
        ),
    );

    x1.set(1f32);
    x2.set(2f32);
    x3.set(3f32);

    let result = g.compute().map(round);
    assert_eq!(result, Some(-0.32727));

    x1.set(2f32);
    x2.set(3f32);
    x3.set(4f32);

    let result = g.compute().map(round);
    assert_eq!(result, Some(-0.56656));
}

pub fn round(x: f32) -> f32 {
    round_to(x, 5)
}

pub fn round_to(x: f32, precision: u32) -> f32 {
    let m = 10i32.pow(precision) as f32;
    (x * m).round() / m
}
