//    .map(|&x| x*x)
//    Map iterates over each element outputting one replacement element each

//    .flat_map(|&x| vec![*x,(*x)+1,(*x)+2])
//    Flat Map iterates over each element outputting potentially multiple elements each, and then flattening it all down into the single dimensional iterator

//    .filter