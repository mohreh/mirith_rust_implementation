use crate::{
    hash::{Hash, Seed},
    params::{N_PARTIES_ROUND, SEED_SIZE, TREE_HEIGHT, TREE_N_NODES},
    prng::Prng,
};

const fn left_child(i: usize) -> usize {
    2 * i + 1
}

const fn right_child(i: usize) -> usize {
    2 * i + 2
}

pub fn generate_subtree(subtree: &mut [Seed], salt: &Hash, root: &Seed, height: usize) {
    let height: u32 = height.try_into().unwrap();
    let mut prng = Prng::new(&Some(*salt), &Some(*root));
    subtree[0].copy_from_slice(root);
    let max_nodes = 1usize.overflowing_shl(height).0 - 1;
    for i in 0..max_nodes {
        prng.gen_bytes(&mut subtree[left_child(i)], SEED_SIZE);
        prng.gen_bytes(&mut subtree[right_child(i)], SEED_SIZE);
    }
}

pub fn get_leaves(subtree: &[Seed], height: u32) -> &[Seed] {
    &subtree[1usize.overflowing_shl(height).0 - 1..]
}

pub fn seed_tree_init(tree: &mut [Seed; TREE_N_NODES], salt: &Hash, seed: &Seed) {
    let mut prng = Prng::new(&Some(*salt), &Some(*seed));
    let mut root = [0u8; SEED_SIZE];
    prng.gen_bytes(&mut root, SEED_SIZE);

    generate_subtree(tree, salt, &root, TREE_HEIGHT);
}

pub fn seed_tree_get_leaves(tree: &[Seed; TREE_N_NODES]) -> &[Seed] {
    &tree[TREE_N_NODES - N_PARTIES_ROUND..]
}

pub fn seed_tree_pack(packed_tree: &mut [Seed], tree: &[Seed; TREE_N_NODES], i0: u32) {
    let mut next_node = 0;

    for (j, item) in packed_tree.iter_mut().enumerate().take(TREE_HEIGHT) {
        if (i0 >> (TREE_HEIGHT - 1 - j)) & 1 != 0 {
            item.copy_from_slice(&tree[left_child(next_node)]);
            next_node = right_child(next_node);
        } else {
            item.copy_from_slice(&tree[right_child(next_node)]);
            next_node = left_child(next_node);
        }
    }
}

pub fn seed_tree_unpack(seeds: &mut [Seed], salt: &Hash, packed_tree: &[Seed], i0: u32) {
    let mut temp_tree = vec![[0u8; SEED_SIZE]; TREE_N_NODES];
    let mut gap_covered = 0;
    let mut number_of_seeds_to_copy = N_PARTIES_ROUND >> 1;

    for j in (0..TREE_HEIGHT).rev() {
        let j_bit = (i0 >> j) & 1;
        let aux_initial_index = (1u32 - j_bit) * (1 << j);
        let initial_seed_index = aux_initial_index as usize + gap_covered as usize;
        gap_covered += j_bit * (1 << j);

        generate_subtree(&mut temp_tree, salt, &packed_tree[TREE_HEIGHT - 1 - j], j);
        seeds[initial_seed_index..initial_seed_index + number_of_seeds_to_copy]
            .copy_from_slice(&get_leaves(&temp_tree, j as u32)[..number_of_seeds_to_copy]);

        number_of_seeds_to_copy /= 2;
    }
}
