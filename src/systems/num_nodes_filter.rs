use crate::systems::filter::{ExtractsFilterableCategories, Filterable};

pub struct NumNodesFilter {
    node_counts: Vec<u16>,
}

impl NumNodesFilter {
    pub fn new(node_counts_as_str: Vec<String>) -> Option<Self> {
        let node_counts: Vec<u16> = match node_counts_as_str.iter().try_fold(
            Vec::new(),
            |mut vec, elem| -> Result<Vec<u16>, ()> {
                let result_parse: u16 = elem.parse().map_err(|_| ())?;

                vec.push(result_parse);
                return Ok(vec);
            },
        ) {
            Ok(val) => val,
            Err(_) => return None,
        };
        Some(Self { node_counts })
    }
}

impl Filterable for NumNodesFilter {
    fn does_job_meet_filter_reqs(&self, job: &dyn ExtractsFilterableCategories) -> bool {
        self.node_counts.contains(&(job.get_num_nodes()))
    }
}
