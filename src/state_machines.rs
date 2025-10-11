prelude!();
pub trait StateMachineBuilder: Send + Sync {
    fn build(&self, sm: StateMachine) -> StateMachine;
}

impl<F> StateMachineBuilder for F
where
    F: Fn(StateMachine) -> StateMachine + Send + Sync + 'static,
{
    fn build(&self, sm: StateMachine) -> StateMachine {
        (self)(sm)
    }
}

pub struct StateMachineFactory;

impl StateMachineFactory {
    pub fn from_many<I>(builders: I) -> Box<dyn StateMachineBuilder>
    where
        I: IntoIterator<Item = Box<dyn StateMachineBuilder + Send + Sync>>,
    {
        struct Seq {
            steps: Vec<Box<dyn StateMachineBuilder + Send + Sync>>,
        }

        impl StateMachineBuilder for Seq {
            fn build(&self, mut sm: StateMachine) -> StateMachine {
                for b in &self.steps {
                    sm = b.build(sm);
                }
                sm
            }
        }

        Box::new(Seq {
            steps: builders.into_iter().collect(),
        })
    }
}
