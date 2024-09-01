use mirith::matrix::MatrixBuilder;

fn main() {
    let matrix_data = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let matrix = MatrixBuilder::<i32>::new()
        .from_vec(4, 2, matrix_data)
        .build()
        .unwrap();

    // Packing the matrix
    // let packed_matrix = matrix.pack();
    println!("Packed Matrix: {:?}", matrix);
    println!("{:?}", matrix[(2, 1)])

    // Unpacking the matrix
    // let unpacked_matrix = Matrix::<u8>::unpack(packed_matrix, 4, 2);
    // println!("Unpacked Matrix:\n{}", unpacked_matrix);
}
