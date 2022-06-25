//! A quorum is a set of nodes a vote request or append-entries request has to contact to.
//! The most common quorum is **majority**.
//! A quorum set is a collection of quorums, e.g. the quorum set of majority of `{a,b,c}` is `{a,b}, {b,c}, {a,c}`.

mod joint;
mod joint_impl;
mod quorum_set;
mod quorum_set_impl;
mod util;

#[cfg(feature = "bench")]
#[cfg(test)]
mod bench;

#[cfg(test)] mod quorum_set_test;
#[cfg(test)] mod util_test;

pub(crate) use joint::AsJoint;
pub(crate) use joint::Joint;
pub(crate) use quorum_set::QuorumSet;
pub(crate) use util::majority_of;
