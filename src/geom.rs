use crate::math::Vec2D;
// From Real-Time Collision Detection by Christer Ericson,
// published by Morgan Kaufmann Publishers, © 2005 Elsevier Inc
// p.139, ClosestPtPointTriangle
fn closest_triangle_point(p: Vec2D, a: Vec2D, b: Vec2D, c: Vec2D) -> Vec2D {
    let (ab, ap) = (b - a, p - a);
    let (bc, bp) = (c - b, p - b);
    let (ca, cp) = (a - c, p - c);
    // Compute parametric position s for projection P’ of P on AB,
    // P’ = A + s*AB, s = c_nom / (c_nom + c_denom)
    let c_nom = ap.dot(ab);
    let c_denom = -bp.dot(ab);
    // Compute parametric position t for projection P’ of P on CA,
    // P’ = A + t*AC, t = b_nom / (b_nom + b_denom)
    let b_nom = cp.dot(ca);
    let b_denom = -ap.dot(ca);
    // Vertex region early out
    if c_nom <= 0.0 && b_denom <= 0.0 {
        return a;
    }
    // Compute parametric position u for projection P’ of P on BC,
    // P’ = B + u*BC, u = a_nom / (a_nom + a_denom)
    let a_nom = bp.dot(bc);
    let a_denom = -cp.dot(bc);
    // Vertex region early outs
    if a_nom <= 0.0 && c_denom <= 0.0 {
        return b;
    }
    if b_nom <= 0.0 && a_denom <= 0.0 {
        return c;
    }
    // P is outside (or on) AB if the triple scalar product [N PA PB] <= 0
    let n = ab.cross(ca);
    let vc = n * ap.cross(bp);
    // If P outside AB and within feature region of AB,
    // return projection of P onto AB
    if vc > 0.0 && c_nom >= 0.0 && c_denom >= 0.0 {
        return a + ab.scale(c_nom / (c_nom + c_denom));
    }
    // P is outside (or on) BC if the triple scalar product [N PB PC] <= 0
    let va = n * bp.dot(cp);
    // If P outside BC and within feature region of BC,
    // return projection of P onto BC
    if va >= 0.0 && a_nom >= 0.0 && a_denom >= 0.0 {
        return b + bc.scale(a_nom / (a_nom + a_denom));
    }
    // P is outside (or on) CA if the triple scalar product [N PC PA] <= 0
    let vb = n * cp.cross(ap);
    // If P outside CA and within feature region of CA,
    // return projection of P onto CA
    if vb >= 0.0 && b_nom >= 0.0 && b_denom >= 0.0 {
        return c + ca.scale(b_nom / (b_nom + b_denom));
    }
    // P must project inside face region. Compute Q using barycentric coordinates
    let a_scale = va / (va + vb + vc);
    let b_scale = vb / (va + vb + vc);
    let c_scale = 1.0 - a_scale - b_scale; // = vc / (va + vb + vc)
    return a.scale(a_scale) + b.scale(b_scale) + c.scale(c_scale);
}

pub fn test_circle_triangle(center: Vec2D, radius: f64, a: Vec2D, b: Vec2D, c: Vec2D) -> bool {
    test_circle_point(center, radius, closest_triangle_point(center, a, b, c))
}

pub fn test_circle_point(center: Vec2D, radius: f64, point: Vec2D) -> bool {
    (center - point).len_squared() <= radius * radius
}
