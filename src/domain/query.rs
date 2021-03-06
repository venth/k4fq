use std::sync::Arc;

use shaku;
use shaku::Component;

use crate::domain::action::Action;
use crate::domain::model::Criteria;
use crate::domain::ports::RecordFinder;
use crate::domain::service::ActionFactory;

impl ActionFactory for QueryActionFactory {
    fn create(&self, criteria: Box<dyn Criteria>, topics: Vec<&str>) -> Box<dyn Action> {
        Box::from(QueryAction {
            record_finder: self.record_finder.clone(),
            criteria: Box::from(criteria),
            topics: topics.iter().map(|e| String::from(*e)).collect(),
        })
    }
}

#[derive(Component)]
#[shaku(interface = ActionFactory)]
pub struct QueryActionFactory {
    #[shaku(inject)]
    record_finder: Arc<dyn RecordFinder>,
}

struct QueryAction {
    record_finder: Arc<dyn RecordFinder>,
    criteria: Box<dyn Criteria>,
    topics: Vec<String>,
}

impl Action for QueryAction {
    fn execute(&self) {
        todo!()
    }
}