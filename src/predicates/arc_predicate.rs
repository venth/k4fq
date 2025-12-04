use std::fmt;
use predicates::{reflection, Predicate};
use std::sync::Arc;

pub struct ArcPredicate<Item: ?Sized>(Arc<dyn Predicate<Item> + Send + Sync>);

impl<Item> ArcPredicate<Item>
where
    Item: ?Sized,
{
    pub fn new<P>(inner: P) -> ArcPredicate<Item>
    where
        P: Predicate<Item> + Send + Sync + 'static,
    {
        ArcPredicate(Arc::new(inner))
    }
}

impl<Item> Clone for ArcPredicate<Item> {
    fn clone(&self) -> Self {
        ArcPredicate(self.0.clone())
    }
}

impl<Item> fmt::Debug for ArcPredicate<Item>
where
    Item: ?Sized,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ArcPredicate").finish()
    }
}

impl<Item> reflection::PredicateReflection for ArcPredicate<Item>
where
    Item: ?Sized,
{
    fn parameters<'a>(&'a self) -> Box<dyn Iterator<Item = reflection::Parameter<'a>> + 'a> {
        self.0.parameters()
    }

    fn children<'a>(&'a self) -> Box<dyn Iterator<Item = reflection::Child<'a>> + 'a> {
        self.0.children()
    }
}

impl<Item> fmt::Display for ArcPredicate<Item>
where
    Item: ?Sized,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<Item> Predicate<Item> for ArcPredicate<Item>
where
    Item: ?Sized,
{
    fn eval(&self, variable: &Item) -> bool {
        self.0.eval(variable)
    }

    fn find_case(&self, expected: bool, variable: &Item) -> Option<reflection::Case> {
        default_find_case(self, expected, variable)
    }
}

fn default_find_case<'a, P, Item>(
    pred: &'a P,
    expected: bool,
    variable: &Item,
) -> Option<reflection::Case<'a>>
where
    P: Predicate<Item>,
    Item: ?Sized,
{
    let actual = pred.eval(variable);
    if expected == actual {
        Some(reflection::Case::new(Some(pred), actual))
    } else {
        None
    }
}

pub trait PredicateArcExt<Item: ?Sized>
where
    Self: Predicate<Item>,
{
    fn arced(self) -> ArcPredicate<Item>
    where
        Self: Sized + Send + Sync + 'static,
    {
        ArcPredicate::new(self)
    }
}

impl<P, Item: ?Sized> PredicateArcExt<Item> for P where P: Predicate<Item> {}

#[cfg(test)]
mod test {
    use predicates::Predicate;
    use predicates::prelude::predicate;
    use crate::predicates::PredicateArcExt;

    #[test]
    fn works_as_unsized_arced() {
        let p = predicate::always().arced();
        p.eval("some value");
    }
}
