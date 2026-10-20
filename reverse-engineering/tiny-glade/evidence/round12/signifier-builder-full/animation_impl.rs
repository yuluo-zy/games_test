impl SignifierAnimation {
    pub fn easing(&self) -> ::utils::easing_functions::EasingFunction {
        use ::utils::easing_functions::EasingFunction as E;
        let tag=u32::from_le_bytes(self.head[..4].try_into().unwrap());
        let value=f32::from_le_bytes(self.head[4..].try_into().unwrap());
        match tag {
            0=>E::CircularInOut, 1=>E::CubicInOut, 2=>E::SineInOut,
            3=>E::Bias(value), 4=>E::ElasticInOut, 5=>E::BackOut, 6=>E::CubicOut,
            _=>panic!("invalid SignifierAnimation easing tag {tag}"),
        }
    }
    /// Original140a69998..140a69a1a, independently checked against RNE.
    pub fn advance(&mut self, delta:f32, marked:bool) {
        self.t=if self.duration!=0.0 {
            let signed_delta=if marked { -delta } else { delta };
            let raw=(1.0/self.duration)*signed_delta+self.t;
            let lower=if 0.0>raw {0.0} else {raw};
            if 1.0<lower {1.0} else {lower}
        } else if marked {0.0} else {1.0};
        self.eased=self.easing().apply(self.t);
    }
}

pub fn signifier_animation_t(
    time: Res<crate::resources::time::GameTime>,
    mut query: Query<(&mut SignifierAnimation,
        Option<&super::signifier_mark_for_delete::SignifierMarkedForDeletion>,
        Option<&mut super::signifier_mark_for_delete::SignifierMinimumLifespan>)>,
) {
    for (mut anim, marked, lifespan) in &mut query {
        anim.advance(time.delta, marked.is_some());
        if let Some(mut lifespan)=lifespan { lifespan.t+=time.delta; }
    }
}
