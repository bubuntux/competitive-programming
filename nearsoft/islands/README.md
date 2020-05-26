# Islands problem

Write a function `int islands(matrix of ints)` that identifies islands in a matrix.
An island is a set of true values linked vertically or horizontally.
Note: true values can *NOT* be linked diagonally.

i.e.  
1,0,0  
1,1,1  
1,0,1  

returns 1 island  

as  
1,   
1,1,1  
1, ,1 represents one island   

Test cases:   

input:   
1,0,1,1,0   
0,1,0,1,0   
1,0,0,0,0  
0,0,1,1,1  

islands = 5   


input:    

1,0,1  
0,1,0  
1,0,1  

islands = 5   


input:   

0,1,1  
0,1,0  
1,1,0  

islands = 1  


input:    

0,0,1,1,0  
1,1,0,1,0  
1,1,0,1,0  
0,0,1,1,1  
0,1,1,0,0  

islands = 2   

int islands(int[][] matrix) {
  return -1;
}
