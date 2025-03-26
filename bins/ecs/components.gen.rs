pub struct PositionComponent(PositionData);

#[automatically_derived]
impl Deref for PositionComponent {
    type Target = PositionData;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[automatically_derived]
impl DerefMut for PositionComponent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub struct VelocityComponent(VelocityData);

#[automatically_derived]
impl Deref for VelocityComponent {
    type Target = VelocityData;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[automatically_derived]
impl DerefMut for VelocityComponent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub struct HealthComponent(HealthData);

#[automatically_derived]
impl Deref for HealthComponent {
    type Target = HealthData;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[automatically_derived]
impl DerefMut for HealthComponent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

