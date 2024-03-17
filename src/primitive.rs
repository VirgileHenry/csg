
#[derive(Debug, Clone, Copy)]
pub enum Primitive {
    Sphere {
        radius: crate::Float,
        offset: crate::Vec3,
    },
    Cube {
        offset: crate::Vec3,
        size: crate::Vec3,
        rotation: glam::Quat,
    },
}

impl Primitive {
    pub(crate) const VAR_COUNT: u32 = 2;

    pub(crate) fn id(&self) -> u32 {
        match self {
            Primitive::Sphere { .. } => 0,
            Primitive::Cube { .. } => 1,
        }
    } 

    pub fn sphere(radius: crate::Float) -> Primitive {
        Primitive::Sphere { 
            radius,
            offset: crate::Vec3::ZERO,
        }
    }

    pub fn at(self, offset: crate::Vec3) -> Primitive {
        match self {
            Primitive::Sphere { radius, .. } => Primitive::Sphere { radius, offset },
            Primitive::Cube { size, rotation, .. } => Primitive::Cube { offset, size, rotation },
            #[allow(unreachable_patterns)]
            _ => panic!("Primitive building error: `at` builder not defined for {self:?}")
        }
    }
}