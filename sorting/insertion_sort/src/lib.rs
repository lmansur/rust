// #### INSERTION SORTING ####
// Input is `seq`, a list of `n` numbers that must be sorted.
// Starts at i = 1 and compares seq[1] with seq[0], swapping places if seq[0] > seq[1]
// Continues the loop until i = n - 1, comparing current `i` with previous indices
// 
// Example:
// seq = [3, 5, 2, 1]
// i: 1
//     i[1] > i[0], so nothing happens.
//     seq = [3, 5, 2, 1]
// i: 2
//     i[2] < i[1], i[2] now is i[1] and previous i[1] is i[2]
//     i[1] < i[0], i[1] now is i[0] and previous i[0] is i[1]
//     seq = [2, 3, 5, 1]
// i: 3
//     i[3] < i[2], i[3] is now i[2] and previous i[2] is i[3]
//     i[2] < i[1], i[2] is now i[1] and previous i[1] is i[2]
//     i[1] < i[0], i[1] is now i[0] and previous i[0] is i[1]
//     seq = [1, 2, 3, 5]
pub fn insertion_sort(seq: &mut [u64]) {
   for i in 1..(seq.len()) { 
       for j in 0..i {
           if seq[i] < seq[j] {
               let tmp = seq[j];
               seq[j] = seq[i];
               seq[i] = tmp;
           }
       }
   }
}
