pub struct BoundingBox {
    pub x_min: f32,
    pub x_max: f32,
    pub y_min: f32,
    pub y_max: f32,
    pub z_min: f32,
    pub z_max: f32,
}

impl BoundingBox {
    pub fn new(mesh: &nom_stl::Mesh) -> BoundingBox {
        let mut bounding_box = BoundingBox {
            x_min: 0.0,
            x_max: 0.0,
            y_min: 0.0,
            y_max: 0.0,
            z_min: 0.0,
            z_max: 0.0,
        };

        for triangle in mesh.triangles() {
            for vertex in triangle.vertices().iter() {
                let (x, y, z) = (vertex[0], vertex[1], vertex[2]);
                if x < bounding_box.x_min {
                    bounding_box.x_min = x;
                }
                if x > bounding_box.x_max {
                    bounding_box.x_max = x;
                }
                if y < bounding_box.y_min {
                    bounding_box.y_min = y;
                }
                if y > bounding_box.y_max {
                    bounding_box.y_max = y;
                }
                if z < bounding_box.z_min {
                    bounding_box.z_min = z;
                }
                if z > bounding_box.z_max {
                    bounding_box.z_max = z;
                }
            }
        }

        bounding_box
    }

    pub fn dx(&self) -> f32 {
        self.x_max - self.x_min
    }

    pub fn dy(&self) -> f32 {
        self.y_max - self.y_min
    }

    pub fn dz(&self) -> f32 {
        self.z_max - self.z_min
    }

    pub fn center(&self) -> cgmath::Point3<f32> {
        let x = self.x_min + self.dx() / 2.0;
        let y = self.y_min + self.dy() / 2.0;
        let z = self.z_min + self.dz() / 2.0;
        cgmath::Point3::new(x, y, z)
    }

    pub fn center_to_origin(&self) -> cgmath::Vector3<f32> {
        let origin = cgmath::Point3::new(0.0, 0.0, 0.0);
        origin - self.center()
    }

    // enum Axis {
    //     X,
    //     Y,
    //     Z,
    // }

    // fn largest_bounding_box_cross_section(bounding_box: &BoundingBox) -> Axis {
    //     let x_area =
    // }
}
