//! A module dedicated to graph generators.
//!
//! Graph generators are responsible for the creation of both inner and outer graphs.
//! Invoking a graph generator with a random generator produces a graph.
//!
//! ```
//! # use crusti_g2io::generators;
//! use rand_core::SeedableRng;
//!
//! // building a generator for Barabási-Albert graphs.
//! let generator = generators::directed_generator_factory_from_str("ba/100,5").unwrap();
//! let mut rng = rand_pcg::Pcg32::from_entropy();
//! // building a graph
//! let g1 = generator(&mut rng);
//! // building another graph with the same generator
//! let g2 = generator(&mut rng);
//! ```

mod barabasi_albert_generator;
pub use barabasi_albert_generator::BarabasiAlbertGeneratorFactory;

mod chain_generator;
pub use chain_generator::ChainGeneratorFactory;

mod erdos_renyi;
pub use erdos_renyi::ErdosRenyiGeneratorFactory;

mod tree_generator;
pub use tree_generator::TreeGeneratorFactory;

mod watts_strogatz;
pub use watts_strogatz::WattsStrogatzGeneratorFactory;

use crate::{core::named_param, Graph, NamedParam};
use anyhow::{Context, Result};
use lazy_static::lazy_static;
use petgraph::{Directed, EdgeType, Undirected};
use rand::Rng;
use rand_pcg::Pcg32;

/// A boxed function that takes a random generator and outputs a graph.
///
/// Such functions are returned by generator factories, and allow the instantiation of graphs.
/// The parameterized type is the one of the random generator.
///
/// ```
/// # use crusti_g2io::generators;
/// use rand_core::SeedableRng;
///
/// // getting a boxed generating function from a string
/// let generator = generators::directed_generator_factory_from_str("chain/3").unwrap();
/// let graph = generator(&mut rand_pcg::Pcg32::from_entropy());
/// ```
pub type BoxedGenerator<Ty, R> = Box<dyn Fn(&mut R) -> Graph<Ty> + Sync + Send>;

/// A trait for objects that produce graph generators.
pub trait GeneratorFactory<Ty, R>: NamedParam<BoxedGenerator<Ty, R>>
where
    R: Rng,
    Ty: EdgeType,
{
}

lazy_static! {
    pub(crate) static ref GENERATOR_FACTORIES_DIRECTED_PCG32: [Box<dyn GeneratorFactory<Directed, Pcg32> + Sync>; 5] = [
        Box::new(BarabasiAlbertGeneratorFactory::default()),
        Box::new(ChainGeneratorFactory::default()),
        Box::new(ErdosRenyiGeneratorFactory::default()),
        Box::new(TreeGeneratorFactory::default()),
        Box::new(WattsStrogatzGeneratorFactory::default()),
    ];
}

lazy_static! {
    pub(crate) static ref GENERATOR_FACTORIES_UNDIRECTED_PCG32: [Box<dyn GeneratorFactory<Undirected, Pcg32> + Sync>; 5] = [
        Box::new(BarabasiAlbertGeneratorFactory::default()),
        Box::new(ChainGeneratorFactory::default()),
        Box::new(ErdosRenyiGeneratorFactory::default()),
        Box::new(TreeGeneratorFactory::default()),
        Box::new(WattsStrogatzGeneratorFactory::default()),
    ];
}

/// Iterates over all the directed graph generator factories.
///
/// ```
/// # use crusti_g2io::generators;
/// generators::iter_directed_generator_factories().enumerate().for_each(|(i,g)| {
///     println!(r#"generator {} has name "{}""#, i, g.name());
/// });
/// ```
pub fn iter_directed_generator_factories(
) -> impl Iterator<Item = &'static (dyn GeneratorFactory<Directed, Pcg32> + Sync + 'static)> + 'static
{
    GENERATOR_FACTORIES_DIRECTED_PCG32
        .iter()
        .map(|b| b.as_ref())
}

/// Iterates over all the undirected graph generator factories.
///
/// ```
/// # use crusti_g2io::generators;
/// generators::iter_undirected_generator_factories().enumerate().for_each(|(i,g)| {
///     println!(r#"generator {} has name "{}""#, i, g.name());
/// });
/// ```
pub fn iter_undirected_generator_factories(
) -> impl Iterator<Item = &'static (dyn GeneratorFactory<Undirected, Pcg32> + Sync + 'static)> + 'static
{
    GENERATOR_FACTORIES_UNDIRECTED_PCG32
        .iter()
        .map(|b| b.as_ref())
}

/// Given a string representing a parameterized directed graph generator factory, returns the corresponding object.
///
/// ```
/// # use crusti_g2io::generators;
/// assert!(generators::directed_generator_factory_from_str("chain/3").is_ok()); // OK
/// assert!(generators::directed_generator_factory_from_str("chain/1,2,3").is_err()); // wrong parameters
/// assert!(generators::directed_generator_factory_from_str("foo/3").is_err()); // unknown generator
/// ```
pub fn directed_generator_factory_from_str(s: &str) -> Result<BoxedGenerator<Directed, Pcg32>> {
    named_param::named_from_str(GENERATOR_FACTORIES_DIRECTED_PCG32.as_slice(), s)
        .context("while building a generator from a string")
}

/// Given a string representing a parameterized undirected graph generator factory, returns the corresponding object.
///
/// ```
/// # use crusti_g2io::generators;
/// assert!(generators::undirected_generator_factory_from_str("chain/3").is_ok()); // OK
/// assert!(generators::undirected_generator_factory_from_str("chain/1,2,3").is_err()); // wrong parameters
/// assert!(generators::undirected_generator_factory_from_str("foo/3").is_err()); // unknown generator
/// ```
pub fn undirected_generator_factory_from_str(s: &str) -> Result<BoxedGenerator<Undirected, Pcg32>> {
    named_param::named_from_str(GENERATOR_FACTORIES_UNDIRECTED_PCG32.as_slice(), s)
        .context("while building a generator from a string")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generator_ok() {
        assert!(directed_generator_factory_from_str("chain/1").is_ok());
    }

    #[test]
    fn test_unknown_generator() {
        assert!(directed_generator_factory_from_str("foo/1").is_err());
    }

    #[test]
    fn test_generator_no_params() {
        assert!(directed_generator_factory_from_str("chain").is_err());
    }
}
