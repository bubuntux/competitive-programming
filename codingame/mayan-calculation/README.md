# Mayan Calculation

<div class="statement-body">

<div class="statement-section statement-goal">

# <span class="icon icon-goal"> </span> <span>The Goal</span>

<div class="statement-goal-content">Upon discovering a new Maya site, hundreds of mathematics, physics and astronomy books have been uncovered.  

The end of the world might arrive sooner than we thought, but we need you to be sure that it doesn't!  

Thus, in order to computerize the mayan scientific calculations, you're asked to develop a program capable of performing basic arithmetical operations between two mayan numbers.</div>

</div>

<div class="statement-section statement-rules">

# <span class="icon icon-rules"> </span> <span>Rules</span>

<div>

<div class="statement-rules-content">The mayan numerical system contains <const>20</const> numerals going from <const>0</const> to <const>19</const>. Here's an ASCII example of their representation:

<table border="1" cellpadding="1" cellspacing="1" style="margin-top: 10px; margin-bottom: 20px; border-collapse: collapse; width: 100%;"><colgroup><col style="width:10%;font-family: monospace !important;"> <col style="width:10%;"> <col style="width:10%;font-family: monospace !important;"> <col style="width:10%;font-family: monospace !important;"> <col style="width:10%;font-family: monospace !important;"> <col style="width:10%;font-family: monospace !important;"> <col style="width:10%;font-family: monospace !important;"> <col style="width:10%;font-family: monospace !important;"> <col style="width:10%;font-family: monospace !important;"> <col style="width:10%;font-family: monospace !important;"></colgroup> 

<tbody>

<tr style="letter-spacing: 2px;">

<td style="text-align: center;font-family: monospace !important;">0</td>

<td style="text-align: center;font-family: monospace !important;">1</td>

<td style="text-align: center;font-family: monospace !important;">2</td>

<td style="text-align: center;font-family: monospace !important;">3</td>

<td style="text-align: center;font-family: monospace !important;">4</td>

<td style="text-align: center;font-family: monospace !important;">5</td>

<td style="text-align: center;font-family: monospace !important;">6</td>

<td style="text-align: center;font-family: monospace !important;">7</td>

<td style="text-align: center;font-family: monospace !important;">8</td>

<td style="text-align: center;font-family: monospace !important;">9</td>

</tr>

<tr style="letter-spacing: 2px;">

<td style="text-align: center;font-family: monospace !important;">.oo.  
o..o  
.oo.  
....</td>

<td style="text-align: center;font-family: monospace !important;">o...  
....  
....  
....</td>

<td style="text-align: center;font-family: monospace !important;">oo..  
....  
....  
....</td>

<td style="text-align: center;font-family: monospace !important;">ooo.  
....  
....  
....</td>

<td style="text-align: center;font-family: monospace !important;">oooo  
....  
....  
....</td>

<td style="text-align: center;font-family: monospace !important;">....  
----  
....  
....</td>

<td style="text-align: center;font-family: monospace !important;">o...  
----  
....  
....</td>

<td style="text-align: center;font-family: monospace !important;">oo..  
----  
....  
....</td>

<td style="text-align: center;font-family: monospace !important;">ooo.  
----  
....  
....</td>

<td style="text-align: center;font-family: monospace !important;">oooo  
----  
....  
....</td>

</tr>

<tr style="letter-spacing: 2px;">

<td style="text-align: center;font-family: monospace !important;">10</td>

<td style="text-align: center;font-family: monospace !important;">11</td>

<td style="text-align: center;font-family: monospace !important;">12</td>

<td style="text-align: center;font-family: monospace !important;">13</td>

<td style="text-align: center;font-family: monospace !important;">14</td>

<td style="text-align: center;font-family: monospace !important;">15</td>

<td style="text-align: center;font-family: monospace !important;">16</td>

<td style="text-align: center;font-family: monospace !important;">17</td>

<td style="text-align: center;font-family: monospace !important;">18</td>

<td style="text-align: center;font-family: monospace !important;">19</td>

</tr>

<tr style="letter-spacing: 2px;">

<td style="text-align: center;font-family: monospace !important;">....  
----  
----  
....</td>

<td style="text-align: center;font-family: monospace !important;">o...  
----  
----  
....</td>

<td style="text-align: center;font-family: monospace !important;">oo..  
----  
----  
....</td>

<td style="text-align: center;font-family: monospace !important;">ooo.  
----  
----  
....</td>

<td style="text-align: center;font-family: monospace !important;">oooo  
----  
----  
....</td>

<td style="text-align: center;font-family: monospace !important;">....  
----  
----  
----</td>

<td style="text-align: center;font-family: monospace !important;">o...  
----  
----  
----</td>

<td style="text-align: center;font-family: monospace !important;">oo..  
----  
----  
----</td>

<td style="text-align: center;font-family: monospace !important;">ooo.  
----  
----  
----</td>

<td style="text-align: center;font-family: monospace !important;">oooo  
----  
----  
----</td>

</tr>

</tbody>

</table>

A mayan number is divided into vertical sections. Each section contains a numeral (from 0 to 19) representing a power of 20\. The lowest section corresponds to 20<sup>0</sup>, the one above to 20<sup>1</sup> and so on.  

Thereby, the example below is : (12 x 20 x 20) + (0 x 20) + 5 = 4805  

![](https://www.codingame.com/fileservlet?id=704058141686)

<div>  
To spice the problem up, the mayans used several dialects, and the graphical representation of the numerals could vary from one dialect to another.  

The representation of the mayan numerals will be given as the input of your program in the form of ASCII characters. You will have to display the result of the operation between the two given mayan numbers. The possible operations are <const>*</const>, <const>/</const>, <const>+</const>, <const>-</const></div>

</div>

</div>

</div>

<div class="statement-section statement-protocol">

# <span class="icon icon-protocol"> </span> <span>Game Input</span>

<div class="blk">

<div class="title">Input</div>

<div class="text">

<span class="statement-lineno">Line 1:</span> the width <var>L</var> and height <var>H</var> of a mayan numeral.

<span class="statement-lineno"><var>H</var> next lines:</span> the ASCII representation of the <const>20</const> mayan numerals. Each line is (<const>20</const> x <var>L</var>) characters long.

<span class="statement-lineno">Next line:</span> the amount of lines <var>S1</var> of the first number.

<span class="statement-lineno"><var>S1</var> next lines:</span> the first number, each line having <var>L</var> characters.

<span class="statement-lineno">Next line:</span> the amount of lines <var>S2</var> of the second number.

<span class="statement-lineno"><var>S2</var> next lines:</span> the second number, each line having <var>L</var> characters.

<span class="statement-lineno">Last line:</span> the operation to carry out: <const>*</const>, <const>/</const>, <const>+</const>, or <const>-</const>

</div>

</div>

<div class="blk">

<div class="title">Output</div>

<div class="text">The result of the operation in mayan numeration, <var>H</var> lines per section, each line having <var>L</var> characters.</div>

</div>

<div class="blk">

<div class="title">Constraints</div>

<div class="text">0 < <var>L</var>, <var>H</var> < 100  
0 < <var>S1</var>, <var>S2</var> < 1000  
The remainder of a division is always 0.  
The mayan numbers given as input will not exceed 2<sup>63</sup>.</div>

</div>

<div class="blk">

<div class="title">Example</div>

<div class="text">

<div class="statement-inout" style="table-layout: fixed;">

<div class="statement-inout-in" style="background-color: white">

<div class="title" style="color:#989898;padding:0">Input</div>

<pre style="line-height:14px;overflow-x: auto;">4 4
.oo.o...oo..ooo.oooo....o...oo..ooo.oooo____o...oo..ooo.oooo____o...oo..ooo.oooo
o..o................____________________________________________________________
.oo.........................................____________________________________
................................................................________________
4
o...
....
....
....
4
o...
....
....
....
+
</pre>

</div>

<div class="statement-inout-out" style="background-color: white">

<div class="title" style="color:#989898;padding:0">Output</div>

<pre style="line-height:14px;overflow-x: auto;">oo..
....
....
....
</pre>

</div>

</div>

</div>

</div>

</div>

</div>