
#include "tetris.h"

#define		FOR(i, len)		for(i = 0; i < len; i++) // Shortcut for loop 

 // Global constant for the number of cordinates in a Tetrimino
static const int CORD_NUM = 3; // Doesn't count the center
static const int B_LEN = 10;
static const int B_HEI = 22;

static const int O_CORDS[3][2] = 
{
	{1, 0},
	{0, 1},
	{1, 1}
};

static const int I_CORDS[3][2] = 
{
	{-1, 0},
	{1, 0},
	{2, 0}
};

static const int S_CORDS[3][2] = 
{
	{-1, 0},
	{0, 1},
	{1, 1}
};

static const int Z_CORDS[3][2] = 
{
	{1, 0},
	{0, 1},
	{-1, 1}
};

static const int L_CORDS[3][2] = 
{
	{-1, 0},
	{1, 0},
	{1, 1}
};

static const int J_CORDS[3][2] = 
{
	{1, 0},
	{-1, 0},
	{-1, 1}
};

static const int T_CORDS[3][2] = 
{
	{-1, 0},
	{0, 1},
	{1, 0}
};

int CORD_LIST[7][3][2]; 

static void 
init_cords(void)
{
	memcpy(&CORD_LIST[0], &O_CORDS, sizeof O_CORDS); 
	memcpy(&CORD_LIST[1], &I_CORDS, sizeof I_CORDS); 
	memcpy(&CORD_LIST[2], &S_CORDS, sizeof S_CORDS);
	memcpy(&CORD_LIST[3], &Z_CORDS, sizeof Z_CORDS);
	memcpy(&CORD_LIST[4], &L_CORDS, sizeof L_CORDS);
	memcpy(&CORD_LIST[5], &J_CORDS, sizeof J_CORDS);
	memcpy(&CORD_LIST[6], &T_CORDS, sizeof T_CORDS);
}

// Creator of my structures
mino 
create_mino(mino *piece)
{
	static int call_num;
	if(!call_num) 
	{
		init_cords();
		call_num++;
	}

	int cent_x = 5;
	int cent_y = 20;

	piece->shape = random() % 7 + 1;
	piece->ori = ori_D;
	
	piece->center[0] = cent_x;
	piece->center[1] = cent_y; 
	
	int i;
	FOR(i, CORD_NUM)
	{
		piece->cords[i][0] = 
			cent_x + CORD_LIST[piece->shape -1 ][i][0];
		
		piece->cords[i][1] = 
			cent_y + CORD_LIST[piece->shape -1 ][i][1];
	}

	return *piece;
}

game 
create_game(game *tetris)
{
	int i, j;
	FOR(i, B_LEN)
	{
		FOR(j, B_HEI)
		{
			tetris->board[i][j].state = empty;
		}
	}

	tetris->curr_mino = 
		create_mino(&tetris->curr_mino);

	FOR(i, CORD_NUM)
	{
		tetris->next_minos[i] = 
			create_mino(&tetris->next_minos[i]);
	}

	tetris->level = 1;
	tetris->drop_speed = 950; 

	tetris->lines = 0;
	tetris->time = 0;
	tetris->score = 0;

	return *tetris;

}


// Turns the piece 90 degree clockwise around its center index
// I shape changes the center index and O shape can't rotate
void 
rotate(mino *piece)
{
	int cent_x = piece->center[0];
	int cent_y = piece->center[1];
	
	int i;
	FOR(i, CORD_NUM)
	{	
		int curr_x = piece->cords[i][0];
		int curr_y = piece->cords[i][1];

		piece->cords[i][0] = cent_x + (curr_y - cent_y);
		piece->cords[i][1] = cent_y - (curr_x - cent_x);

	}
	
	piece->ori = ++piece->ori % 4;
	
	if(piece->shape == shape_I && piece->ori % 2)
	{
		int temp_y = piece->center[1];
		piece->center[1] = piece->cords[1][1];
		piece->cords[1][1] = temp_y;
	}
}

int  
can_rotate(game *tetris)
{
	mino *piece = &tetris->curr_mino;

	if(piece->shape == shape_O) return 0;

	int cent_x = piece->center[0];
	int cent_y = piece->center[1];
	
	int i;
	FOR(i, CORD_NUM)
	{	
		int curr_x = piece->cords[i][0];
		int curr_y = piece->cords[i][1];

		int new_x = cent_x + (curr_y - cent_y);
		int new_y = cent_y - (curr_x - cent_x);

		if(new_x >= B_LEN || new_x < 0 || new_y < 0)
		{
			return 0;	
		}

		else if(tetris->board[new_x][new_y].state == placed)
		{
			return 0;
		}
	}
	
	return 1;
}

// Moves the piece left or right one based on the value of direction being + or -
void 
shift(mino *piece, int dir)
{	
	if(!(dir * dir - 1))
	{	
		piece->center[0] += dir; 

		int i;
		FOR(i, CORD_NUM)
		{
			piece->cords[i][0] += dir;
		}
	}
}


int
can_shift(game *tetris, int dir)
{
	if(dir * dir - 1) return 0;

	mino *piece = &tetris->curr_mino;
	
	int cx = piece->center[0];
	int cy = piece->center[1];

	if( cx + dir >= B_LEN || cx + dir < 0 || 
			tetris->board[cx + dir][cy].state == placed)
	{
		return 0;
	}
	
	int i;
	FOR(i, CORD_NUM)
	{
		int x = piece->cords[i][0];
		int y = piece->cords[i][1];

		if( x + dir >= B_LEN || x + dir < 0 || 
				tetris->board[x+dir][y].state == placed)
		{
			return 0;
		}
	}
	
	return 1;
}


// Moves the piece down one
void 
fall(mino *piece)
{
	piece->center[1] -= 1; 

	int i;
	FOR(i, CORD_NUM)
	{
		piece->cords[i][1] -= 1;
	}

}	

int
can_fall(game *tetris)
{
	mino *piece = &tetris->curr_mino;
	
	int cx = piece->center[0];
	int cy = piece->center[1];

	if( cy - 1 < 0 || tetris->board[cx][cy-1].state == placed)
	{
		return 0;
	}
	
	int i;
	FOR(i, CORD_NUM)
	{
		int x = piece->cords[i][0];
		int y = piece->cords[i][1];
		
		if(x == cx && y > cy)
		{
			continue;
		}

		else if( y - 1 < 0 || tetris->board[x][y-1].state == placed)
		{
			return 0;
		}
	}
	
	return 1;
}


// The updates the location of the current tetrimino 
void 
update_pos(game *tetris)
{
	int drop = drop_distance(tetris);
	mino *piece = &tetris->curr_mino;

	int i, j;
	FOR(i, B_LEN)
	{
		FOR(j, B_HEI)
		{
			if(tetris->board[i][j].state == falling || 
					tetris->board[i][j].state == phantom)
			{
				tetris->board[i][j].state = empty;
				tetris->board[i][j].color = empty;
			}
		}
	}
	
	int x = piece->center[0];
	int y = piece->center[1];

	tetris->board[x][y - drop].state = phantom;
	tetris->board[x][y].state = falling;
	tetris->board[x][y].color = piece->shape;


	FOR(i, CORD_NUM)
	{
		x = piece->cords[i][0];
		y = piece->cords[i][1];
	
		tetris->board[x][y - drop].state = phantom;
		tetris->board[x][y].state = falling;
		tetris->board[x][y].color = piece->shape;
	}
}


void
place(game *tetris)
{
	int x = tetris->curr_mino.center[0];
	int y = tetris->curr_mino.center[1];

	tetris->board[x][y].state = placed;
	
	int i;
	FOR(i, CORD_NUM)
	{
		x = tetris->curr_mino.cords[i][0];
		y = tetris->curr_mino.cords[i][1];

		tetris->board[x][y].state = placed;
	}
}



// Releases the next mino making it the current Tetrimino, in the invisible 
// rows of the board (above row 20)
void 
release_next(game *tetris)
{
	tetris->curr_mino = 
		tetris->next_minos[0];
	
	tetris->next_minos[0] = 
		tetris->next_minos[1];
	
	tetris->next_minos[1] = 
		tetris->next_minos[2];
	
	tetris->next_minos[2] = 
		create_mino(&tetris->next_minos[2]);
}


// Gets the distance the current Tetrimino will drop if dropped
int
drop_distance(game *tetris)
{	
	mino piece = tetris->curr_mino;
	
	int i, j, min_move = B_HEI;

	int cx = piece.center[0];
	int cy = piece.center[1];

	for(j = cy; j >= 0; j--)
    {
        if(tetris->board[cx][j].state == placed)
        {
            if(cy -j -1 < min_move)
            {
                min_move = cy -j -1;
            }
			break;
        }

		else if(j == 0 && cy < min_move)
		{
			min_move = cy;
		}
    }

	FOR(i, CORD_NUM)
	{
		int x = piece.cords[i][0];
		int y = piece.cords[i][1];

		if (x != cx || y < cy)
		{
			for(j = y; j >= 0; j--)
			{
				if(tetris->board[x][j].state == placed)
				{
					if(y -j -1 < min_move)
					{
						min_move = y -j -1;
					}
					
					break;
				}

				else if(j == 0 && y < min_move)
				{
					min_move = y;
				}
			}
		}
	}

	return min_move;
}


// Drops the current Tetrimino on top of the pieces below it
void 
drop(game *tetris)
{
	int dist = drop_distance(tetris);
	mino *piece = &tetris->curr_mino;

	piece->center[1] -= dist;
	
	int i;
	FOR(i, CORD_NUM)
	{
		piece->cords[i][1] -= dist;
	}

	update_pos(tetris);
}


// Stores the current Tetrimino and releases the previously held Tetrimino 
// if there is one. Otherwise releases the next Tetrimino 
void 
hold(game *tetris)
{
	int cent_x = 5;
	int cent_y = 20;

	mino *piece = &tetris->held_mino;

	static int count;

	if(count)
	{
		mino temp = tetris->curr_mino;
		tetris->curr_mino = tetris->held_mino;
		tetris->held_mino = temp;
	}
	
	else
	{
		tetris->held_mino = tetris->curr_mino;
		release_next(tetris);
		++count;
	}	

	piece->center[0] = cent_x;
	piece->center[1] = cent_y; 
	
	int i;
	FOR(i, CORD_NUM)
	{
		piece->cords[i][0] = 
			cent_x + CORD_LIST[piece->shape -1 ][i][0];
		
		piece->cords[i][1] = 
			cent_y + CORD_LIST[piece->shape -1 ][i][1];
	}

	update_pos(tetris);
}


static void
shift_all_down(game *tetris, int bottom)
{
	int i, j;
	for(j = bottom; j < B_HEI -1; j++)
	{
		FOR(i, B_LEN)
		{
			tetris->board[i][j] = tetris->board[i][j+1];
		}
	}
}


// Reads the possible rows to clear and if they are full clears
void
line_up(game *tetris)
{
	int i, j; 
	int count, num_rows = 0;
	
	FOR(j, B_HEI)
	{	
		count = 0;

		FOR(i, B_LEN)
		{
			count += tetris->board[i][j].state;
		}
		
		if(count == B_LEN * placed)
		{
			shift_all_down(tetris, j);

			tetris->lines++;
			num_rows++;
			j--;
		}
		
		if(num_rows == 4) break;
	}

	if(tetris->lines / ( 5 * ( tetris->level + 1 ) ) )
	{
		tetris->level++;
		tetris->lines = 0;
		calc_speed(tetris);
	}
}


// Calculates the speed based on what the current level is
void
calc_speed(game *tetris)
{	
	int speed = 950;
	int level = tetris->level;
	tetris->drop_speed = speed * (.95 - (level -1) / level);
}

