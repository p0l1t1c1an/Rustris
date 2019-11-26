
#pragma once

#include <stdlib.h>
#include <string.h>
// Possible shapes 
enum 
{
	shape_O = 1, shape_I, shape_S, shape_Z, shape_L, shape_J, shape_T 

}
mino_shapes;


// Possible orientations Up, Left, Right and Down
enum 
{
	ori_U, ori_L, ori_R, ori_D 

}
mino_orientations;

enum
{
	empty, falling, placed, phantom
}
block_states;

// Struct of a Tetrimino 
typedef
struct Mino
{
	// Ints to hold the values of the shape and orientation
	// Values are only to be thoses in the corresponding enum
	int shape;
	int ori;
	
	// The center cordinate of the Tetrimino
	// Is used as the origin of rotation
	// I shape swaps its cordinate depending on the orientation
	int center[2];

	// 2D array that holds the cordinates of spaces the Tetrimino is in
	// except for the center cord
	int cords[3][2];

}
mino;

typedef 
struct Block
{
	unsigned char state;	// 2 bytes for u_char vs 8 bytes for integers
	unsigned char color;
}
block;


// Struct of the Game
// Holds the info of everything used in the game of Tetris
typedef
struct Game
{
	// 2D array that holds whether a that spaces is taken or not on the game board
	block board[10][22];
	
	// Tetriminos that are to be displayed in the game
	mino curr_mino;
	mino next_minos[3]; // The next the 3 Tetriminos that will be dropped 
	mino held_mino;
	
	// Invisble componenets of Tetris 
	int level;
	int drop_speed; // Called speed but actually Milliseconds

	// Displayed values for the game
	int lines;
	int time;	// Total Milliseconds
	int score;

}
game;


// Creator of my structures
mino create_mino(mino *piece);
game create_game(game *tetris);


// Turns the piece 90 degree clockwise around its center index
// I shape changes the center index and O shape can't rotate
void rotate(mino *piece);

// Check if able to rotate
int can_rotate(game *tetris);

// Moves the piece left or right one based on the value of direction being + or -
void shift(mino *piece, int direction);

int can_shift(game *tetris, int direction);

// Moves the piece down one 
void fall(mino *piece);

int can_fall(game *tetris);

// The current Tetrimino position is saved on the board 
void stop_mino(game *tetris);


// The updates the location of the current tetrimino 
void update_pos(game *tetris);


void place(game *tetris);

// Releases the next mino making it the current Tetrimino, in the invisible 
// rows of the board (above row 20)
void release_next(game *tetris);


// Gets the location of the center index if the current Tetrimino were to be dropped
 int drop_distance(game *tetris);


// Drops the current Tetrimino on top of the pieces below it
void drop(game *tetris);


// Stores the current Tetrimino and releases the previously held Tetrimino 
// if there is one. Otherwise releases the next Tetrimino 
void hold(game *tetris);

// Reads the possible rows to clear and if they are full clears
void line_up(game *tetris);


// Calculates the speed based on what the current level is
void calc_speed(game *tetris);
