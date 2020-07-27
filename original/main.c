
#include "tetris.h"
#include <ncurses.h>
#include <unistd.h>
#include <pthread.h>
#include <time.h>

#define FOR(i, len) for(i = 0; i < len; i++)

static const char BLOCK = ' ';	//Fill in background with color to show block

static const int COLOR_GREY = 17; // Out of predifined terminal all colors ways
static const int COLOR_ORANGE = 18; 

static const int S_LEN = 90, B_LEN = 10, H_LEN = 6, N_LEN = 6;  
static const int S_HEI = 30, B_HEI = 20, H_HEI = 4, N_HEI = 10;

static const int PAD_V = 4,  PAD_H = 4; //vertical and horizontal padding


void 
init_board(void)
{
	init_color(COLOR_GREY, 656, 656, 656);
	init_pair(COLOR_GREY, COLOR_GREY, COLOR_GREY);

	attron(COLOR_PAIR(COLOR_GREY));
	
	int i, j;
	FOR(i, S_LEN)
	{
		FOR(j, S_HEI)
		{	
			mvaddch(j, i, BLOCK);
		}
	}

	attroff(COLOR_PAIR(COLOR_GREY));
/*	
	attron();
	FOR(i, 4)
	{
		FOR(j, 4)
		{
			
		}

	}




	attron(COLOR_PAIR(0));
	refresh();


*/
}



void 
init(void)
{
	initscr();

	curs_set(0);
	cbreak();
	noecho();

	keypad(stdscr, 1);

	start_color();
	
	init_color(COLOR_ORANGE, 1000, 550, 0);

	init_pair(shape_O, COLOR_BLACK, COLOR_YELLOW);
	init_pair(shape_I, COLOR_BLACK, COLOR_ORANGE);
	init_pair(shape_S, COLOR_BLACK, COLOR_GREEN);
	init_pair(shape_Z, COLOR_BLACK, COLOR_RED);
	init_pair(shape_L, COLOR_BLACK, COLOR_CYAN);
	init_pair(shape_J, COLOR_BLACK, COLOR_BLUE);
	init_pair(shape_T, COLOR_BLACK, COLOR_MAGENTA);
	
	init_board();
}


void
draw_next()
{

}


void 
draw_held()
{

}


void 
draw_board(void)
{
	
}





void 
update_lineup(game *tetris)
{
	int i, j;
	FOR(i, B_LEN)
	{
		FOR(j, B_HEI)
		{
			int color = tetris->board[i][j].color;
			
			int x = 2 *i + S_LEN/2 - B_LEN;
			int y = B_HEI - j - S_HEI/2 + B_HEI;

			if(tetris->board[i][j].state == placed)
			{
				attron(COLOR_PAIR(color));

				mvaddch(y, x-1, BLOCK);
				mvaddch(y, x,	BLOCK);

				attroff(COLOR_PAIR(color));
			}
		}
	}
}


void 
update_screen(game *tetris)
{
	int shape = tetris->curr_mino.shape;

	int i, j;
	FOR(i, B_LEN)
	{
		FOR(j, B_HEI)
		{
			int x = 2 *i + S_LEN/2 - B_LEN;
			int y = B_HEI - j - S_HEI/2 + B_HEI;

			if(tetris->board[i][j].state == falling || tetris->board[i][j].state == phantom)
			{
				attron(COLOR_PAIR(shape));

				mvaddch(y, x-1,	BLOCK);
				mvaddch(y, x,	BLOCK);

				attroff(COLOR_PAIR(shape));
			}

			else if(tetris->board[i][j].state == empty)
			{
				attron(COLOR_PAIR(0));
				
				mvaddch(y, x-1,	BLOCK);
				mvaddch(y, x,	BLOCK);

				attroff(COLOR_PAIR(0));
			}
		}
	}
	
	refresh();
}


void *
update_thread(void *arg_game)
{
	game *tetris = (game *)arg_game;
	int *delay = &tetris->drop_speed;
	
	int count = 0;
	while(1)
    {
		update_pos(tetris);
        
		usleep(*delay * 1000);
		
		update_screen(tetris);

		if(can_fall(tetris))
		{
			fall(&tetris->curr_mino);
		}
	
		else
		{
			usleep(1000);
			
			place(tetris);
			line_up(tetris);

			update_lineup(tetris);
			
			release_next(tetris);

			update_pos(tetris);
			update_screen(tetris);
		}
    }
	
	pthread_exit(NULL);
    return NULL;
}


int 
main(void)
{
	srandom(time(NULL));
	game tetris = create_game(&tetris);

	init();

	pthread_t updater;
	pthread_create(&updater, NULL, &update_thread, &tetris);

	int input;

	while((input = getch()) != 'q')
	{			
		if(input == KEY_LEFT && can_shift(&tetris, -1))
		{
			shift(&tetris.curr_mino, -1);
		}
		
		else if(input == KEY_RIGHT && can_shift(&tetris, 1))
		{
			shift(&tetris.curr_mino, 1);
		}
	
		else if(input == KEY_UP && can_rotate(&tetris))
		{
			rotate(&tetris.curr_mino);
		}
		
		else if(input == KEY_DOWN)
		{
			drop(&tetris);
		}

		else if(input == KEY_PPAGE || input == KEY_NPAGE)
		{
			hold(&tetris);
		}
		
		update_pos(&tetris);
		update_screen(&tetris);

		usleep(75000);
	}
	
	endwin();
	printf("%d, %d", tetris.drop_speed, tetris.level);
		
	return 0;
}
	
