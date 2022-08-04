use crate::math::Vec2D;

fn closest_triangle_point(p: Vec2D, a: Vec2D, b: Vec2D, c: Vec2D) -> Vec2D {
    // adapted from Real-Time Collision Detection by Christer Ericson,
    // published by Morgan Kaufmann Publishers, © 2005 Elsevier Inc
    // p.139, ClosestPtPointTriangle

    let (ab, ap) = (b - a, p - a);
    let (bc, bp) = (c - b, p - b);
    let (ca, cp) = (a - c, p - c);

    // check vertex regions
    // Compute parametric position s for projection P’ of P on AB,
    // P’ = A + s*AB, s = c_nom / (c_nom + c_denom)
    let c_nom = ap.dot(ab);
    let c_denom = -bp.dot(ab);
    // Compute parametric position t for projection P’ of P on CA,
    // P’ = C + t*CA, t = b_nom / (b_nom + b_denom)
    let b_nom = cp.dot(ca);
    let b_denom = -ap.dot(ca);
    if c_nom <= 0.0 && b_denom <= 0.0 {
        return a;
    }
    // Compute parametric position u for projection P’ of P on BC,
    // P’ = B + u*BC, u = a_nom / (a_nom + a_denom)
    let a_nom = bp.dot(bc);
    let a_denom = -cp.dot(bc);
    if a_nom <= 0.0 && c_denom <= 0.0 {
        return b;
    }
    if b_nom <= 0.0 && a_denom <= 0.0 {
        return c;
    }

    // check edge regions
    // P is outside or on AB if the triple scalar product [N AP BP] >= 0
    let n = ab.cross(ca);
    let vc = n * ap.cross(bp);
    if vc >= 0.0 && c_nom >= 0.0 && c_denom >= 0.0 {
        return a + ab.scale(c_nom / (c_nom + c_denom));
    }
    // P is outside or on BC if the triple scalar product [N BP CP] >= 0
    let va = n * bp.cross(cp);
    if va >= 0.0 && a_nom >= 0.0 && a_denom >= 0.0 {
        return b + bc.scale(a_nom / (a_nom + a_denom));
    }
    // P is outside or on CA if the triple scalar product [N CP AP] >= 0
    let vb = n * cp.cross(ap);
    if vb >= 0.0 && b_nom >= 0.0 && b_denom >= 0.0 {
        return c + ca.scale(b_nom / (b_nom + b_denom));
    }

    // P must be inside face region
    p
}

pub fn test_circle_triangle(center: Vec2D, radius: f64, a: Vec2D, b: Vec2D, c: Vec2D) -> bool {
    test_circle_point(center, radius, closest_triangle_point(center, a, b, c))
}

pub fn test_circle_point(center: Vec2D, radius: f64, point: Vec2D) -> bool {
    (center - point).len_squared() <= radius * radius
}
