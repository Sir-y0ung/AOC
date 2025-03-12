#include <iostream>
#include <fstream>
#include <string.h>
#include <list>

using namespace std;

class MapSolver {
    #define wall '#'
    #define box  'O'
    #define free_space '.'
    #define player '@'

    private:
    struct Node {
        int cord_x;
        int cord_y;
        char type_of_node;
    };

    const static int max_x = 50;
    const static int max_y = 50;

    Node* map[max_x][max_y];
    list<char> moves;

    int curr_x = 0;
    int curr_y = 0;
    Node *player_node;
    bool found_wall;

    public:
    MapSolver() {
        memset(map, 0, max_x * max_y * sizeof(int *));
    }


    void map_parser(FILE *map_fd) {

        char ch;
        while ((ch = fgetc(map_fd)) != EOF) {               //map parser

            switch (ch) {
                case '#': {
                    Node * wall_node = (struct Node *)malloc(sizeof(struct Node));
                    wall_node->cord_x = curr_x;
                    wall_node->cord_y = curr_y;
                    wall_node->type_of_node = wall;
                    map[curr_y][curr_x] = wall_node;
                    curr_x++;
                    break;
                }

                case 'O': {
                    Node * box_node = (struct Node *)malloc(sizeof(struct Node));
                    box_node->cord_x = curr_x;
                    box_node->cord_y = curr_y;
                    box_node->type_of_node = box;
                    map[curr_y][curr_x] = box_node;
                    curr_x++;
                    break;
                }

                case '.': {
                    Node * space_node = (struct Node *)malloc(sizeof(struct Node));
                    space_node->cord_x = curr_x;
                    space_node->cord_y = curr_y;
                    space_node->type_of_node = free_space;
                    map[curr_y][curr_x] = space_node;
                    curr_x++;
                    break;
                }

                case '@': {
                    Node * play_node = (struct Node *)malloc(sizeof(struct Node));
                    play_node->cord_x = curr_x;
                    play_node->cord_y = curr_y;
                    play_node->type_of_node = player;
                    map[curr_y][curr_x] = play_node;
                    player_node = play_node;
                    curr_x++;
                    break;
                }

                case '\n': {
                    curr_y++;
                    curr_x = 0;
                }
            }

        }
    }


    void instruction_parser(FILE *move_fd) {
        char ch;

        while ((ch = fgetc(move_fd)) != EOF) {               //map parser
            switch (ch) {
                case '\n': {
                    break;
                }
                default:
                    moves.push_back(ch);
            }
        }
    }
    void move_up(Node *pos_to_go) {

        if (found_wall) {
            if (pos_to_go->type_of_node == wall || (map[pos_to_go->cord_y-1][pos_to_go->cord_x]->type_of_node == wall && pos_to_go->type_of_node == box)) {
                found_wall = false;
                return;
            }
            if (pos_to_go->type_of_node == free_space) {
                map[pos_to_go->cord_y][pos_to_go->cord_x]->type_of_node = map[pos_to_go->cord_y+1][pos_to_go->cord_x]->type_of_node;

                if (map[pos_to_go->cord_y+1][pos_to_go->cord_x]->type_of_node == '@') {
                    player_node = map[pos_to_go->cord_y][pos_to_go->cord_x];
                }
                map[pos_to_go->cord_y+1][pos_to_go->cord_x]->type_of_node = free_space;
                return;
            }
            if (pos_to_go->type_of_node == box) {
                move_up(map[pos_to_go->cord_y-1][pos_to_go->cord_x]);
            }
            move_up(map[pos_to_go->cord_y][pos_to_go->cord_x]);
        }
    }

    void move_right(Node *pos_to_go) {

        if (found_wall) {
            if (pos_to_go->type_of_node == wall || (map[pos_to_go->cord_y][pos_to_go->cord_x+1]->type_of_node == wall && pos_to_go->type_of_node == box)) {
                found_wall = false;
                return;
            }
            if (pos_to_go->type_of_node == free_space) {
                map[pos_to_go->cord_y][pos_to_go->cord_x]->type_of_node = map[pos_to_go->cord_y][pos_to_go->cord_x-1]->type_of_node;

                if (map[pos_to_go->cord_y][pos_to_go->cord_x-1]->type_of_node == '@') {
                    player_node = map[pos_to_go->cord_y][pos_to_go->cord_x];
                }
                map[pos_to_go->cord_y][pos_to_go->cord_x-1]->type_of_node = free_space;
                return;
            }
            if (pos_to_go->type_of_node == box) {
                move_right(map[pos_to_go->cord_y][pos_to_go->cord_x+1]);
            }
            move_right(map[pos_to_go->cord_y][pos_to_go->cord_x]);
        }
    }

    void move_left(Node *pos_to_go) {

        if (found_wall) {
            if (pos_to_go->type_of_node == wall || (map[pos_to_go->cord_y][pos_to_go->cord_x-1]->type_of_node == wall && pos_to_go->type_of_node == box)) {
                found_wall = false;
                return;
            }
            if (pos_to_go->type_of_node == free_space) {
                map[pos_to_go->cord_y][pos_to_go->cord_x]->type_of_node = map[pos_to_go->cord_y][pos_to_go->cord_x+1]->type_of_node;

                if (map[pos_to_go->cord_y][pos_to_go->cord_x+1]->type_of_node == '@') {
                    player_node = map[pos_to_go->cord_y][pos_to_go->cord_x];
                }
                map[pos_to_go->cord_y][pos_to_go->cord_x+1]->type_of_node = free_space;
                return;
            }
            if (pos_to_go->type_of_node == box) {
                move_left(map[pos_to_go->cord_y][pos_to_go->cord_x-1]);
            }
            move_left(map[pos_to_go->cord_y][pos_to_go->cord_x]);
        }
    }

    void move_down(Node *pos_to_go) {
        if (found_wall) {
            if (pos_to_go->type_of_node == wall || (map[pos_to_go->cord_y+1][pos_to_go->cord_x]->type_of_node == wall && pos_to_go->type_of_node == box)) {
                found_wall = false;
                return;
            }
            if (pos_to_go->type_of_node == free_space) {
                map[pos_to_go->cord_y][pos_to_go->cord_x]->type_of_node = map[pos_to_go->cord_y-1][pos_to_go->cord_x]->type_of_node;

                if (map[pos_to_go->cord_y-1][pos_to_go->cord_x]->type_of_node == '@') {
                    player_node = map[pos_to_go->cord_y][pos_to_go->cord_x];
                }
                map[pos_to_go->cord_y-1][pos_to_go->cord_x]->type_of_node = free_space;
                return;
            }
            if (pos_to_go->type_of_node == box) {
                move_down(map[pos_to_go->cord_y+1][pos_to_go->cord_x]);
            }
            move_down(map[pos_to_go->cord_y][pos_to_go->cord_x]);
        }


    }


    void run() {

        for (char move : moves) {
            found_wall = true;

            switch (move) {
                case '^': {
                    move_up(map[player_node->cord_y-1][player_node->cord_x]);
                    break;
                }
                case '>': {
                    move_right(map[player_node->cord_y][player_node->cord_x+1]);
                    break;
                }
                case '<': {
                    move_left(map[player_node->cord_y][player_node->cord_x-1]);
                    break;
                }
                case 'v': {
                    move_down(map[player_node->cord_y+1][player_node->cord_x]);
                    break;
                }
            }


        }
    }

    void summarize_gps() {
        int sum = 0;
        for (int i = 0; i < max_y;i++) {
            for (int j = 0; j < max_y;j++) {
                if (map[i][j]->type_of_node == 'O') {
                    sum += 100 * map[i][j]->cord_y + map[i][j]->cord_x;
                }

            }
        }
        printf("%d", sum);
    }

    void print_map() {
        for (int i = 0; i < max_x; i++) {
            for (int j =0;j< max_y;j++) {
                printf("%c", map[i][j]->type_of_node);
            }
            printf("\n");
        }
    }

};



int main() {
    MapSolver solver = MapSolver();
    FILE* map_fd = fopen("D:\\Coddingg\\C\\AOC2024\\day15\\map.txt", "r");
    FILE* move_fd = fopen("D:\\Coddingg\\C\\AOC2024\\day15\\moves.txt", "r");


    solver.map_parser(map_fd);
    solver.instruction_parser(move_fd);
                                    // sol for part1
    printf("Starting Map\n");
    solver.print_map();

    solver.run();

    printf("Finished Map\n");
    solver.print_map();

    printf("Part1 Solution: ");
    solver.summarize_gps();

                                    // sol for part2

    return 0;
}
