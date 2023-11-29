use crate::subview::*;
use std::rc::Rc;
use std::slice::Iter;
use yew::prelude::*;
use yewdux::prelude::*;

/// The subview stack.
#[derive(Clone, PartialEq, Default, Store)]
pub struct SubviewStack(Vec<Subview>);

impl SubviewStack {
    /// Replaces the current subview with the specified subview. If the
    /// subview stack is empty, the specified subview will be pushed onto the
    /// stack.
    pub fn set<S>(&mut self, subview: S)
    where
        S: Into<Subview>,
    {
        match self.0.last_mut() {
            Some(current) => *current = subview.into(),
            None => self.push(subview),
        }
    }

    /// Pushes the specified subview onto the stack.
    pub fn push<S>(&mut self, subview: S)
    where
        S: Into<Subview>,
    {
        self.0.push(subview.into());
    }

    /// Pops the top-most subview from the stack. If the stack is empty, this
    /// does nothing.
    pub fn pop(&mut self) {
        self.0.pop();
    }

    /// Clears the subview stack. This is usually a good idea when switching
    /// actual views, since otherwise the view change will be invisible
    /// underneath the stacked subviews.
    pub fn clear(&mut self) {
        self.0.clear();
    }

    /// Returns an iterator over the subviews.
    pub fn iter(&self) -> Iter<'_, Subview> {
        self.0.iter()
    }
}

/// A handle to the application's current subview stack.
#[derive(Clone)]
pub struct UseSubviewHandle {
    /// The inner value.
    value: Rc<SubviewStack>,
    /// The dispatcher for the inner value.
    dispatch: Dispatch<SubviewStack>,
}

impl UseSubviewHandle {
    /// Replaces the current subview with the specified subview. If the
    /// subview stack is empty, the specified subview will be pushed onto the
    /// stack.
    pub fn set<S>(&self, subview: S)
    where
        S: Into<Subview>,
    {
        self.dispatch.reduce_mut(|stack| stack.set(subview))
    }

    /// Pushes the specified subview onto the stack.
    pub fn push<S>(&self, subview: S)
    where
        S: Into<Subview>,
    {
        self.dispatch.reduce_mut(|stack| stack.push(subview))
    }

    /// Pops the top-most subview from the stack. If the stack is empty, this
    /// does nothing.
    pub fn pop(&self) {
        self.dispatch.reduce_mut(|stack| stack.pop())
    }

    /// Clears the subview stack. This is usually a good idea when switching
    /// actual views, since otherwise the view change will be invisible
    /// underneath the stacked subviews.
    pub fn clear(&self) {
        self.dispatch.reduce_mut(|stack| stack.clear())
    }

    /// Returns an iterator over the subviews.
    pub fn iter(&self) -> Iter<'_, Subview> {
        self.value.iter()
    }
}

/// Gets a handle to the current subview stack.
#[hook]
pub fn use_subview() -> UseSubviewHandle {
    let (subview, dispatch_subview) = use_store::<SubviewStack>();

    UseSubviewHandle {
        value: subview,
        dispatch: dispatch_subview,
    }
}
