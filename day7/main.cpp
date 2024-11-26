#include <iostream>
#include <bits/stdc++.h>
#include<list>
#include <utility>

using namespace std;

// Question 1
int answer1 = 0;

// Question 2
long total_disk = 70000000;
long disk_in_use = 0;
long long required_space;
long update_size = 30000000;
long answer2 = 70000000;
string dir_name;

class FsNode {  // represent one node on the filesystem like a file or directory

    public:
    int len;
    int num_children;
    string name;

    FsNode* dad;
    list<FsNode *> children;


    FsNode(FsNode *parent, string name, int len) {
        if (len) {
            this->len = len;
        }
        else {
            this->len = -1;
        }

        dad = parent;
        this->name = std::move(name);
        this->num_children = 0;
    }

    void addChild(FsNode* child) {
        children.push_back(child);
    }

    FsNode* getDad() const {
        return dad;
    }

    FsNode* getKid(int index) {
        list<FsNode *>::iterator it = children.begin();
        advance(it, index);
        return *it;
    }


};

FsNode* root_node;
FsNode* curr_node;

void create_fs(list<string> commands) { // creating a general tree representing the filesystem

    for (string cmd : commands) {

        if (cmd.starts_with("$ cd ..")) {
            curr_node = curr_node->getDad();
        }
        else if (cmd.starts_with("$ cd")) {

            int i = 0;
            string dir_name = cmd.substr(strlen("$ cd "));

            if (dir_name == "/") {
                curr_node = new FsNode(nullptr, dir_name, 0);
                root_node = curr_node;
                continue;
            }
            while (i < curr_node->num_children) {
                if (curr_node->getKid(i)->name == dir_name) {
                    curr_node = curr_node->getKid(i);
                    break;
                }
                i++;
            }

        }
        else {
            if (cmd.starts_with("dir ")) {
                curr_node->addChild(new FsNode(curr_node, cmd.substr(strlen("dir ")), 0));
                curr_node->num_children++;
            }
            else {
                std::istringstream stream(cmd);
                std::string filename;
                int size;

                stream >> size >> filename;

                curr_node->addChild(new FsNode(curr_node, filename, size));
                curr_node->num_children++;
            }
        }
    }
}

void sum_fs(FsNode* node) { // sum all files of filesystem to find space on the disk
    if (node == nullptr)
        return;
    int sum = 0;

    for (FsNode* child : node->children) {

        if (child->len != -1)
            sum += child->len;

        sum_fs(child);
    }

    if (sum)
        disk_in_use += sum;
}

int  search_4_delete(FsNode* node) { // summing all directories on a recursion format and compute the sum
    if (node == nullptr)
        return 0;

    int sum = 0;

    for (FsNode* child : node->children) {

        if (child->len != -1)
            sum += child->len;

        sum += search_4_delete(child);
    }
    if (sum <= 100000)
        answer1 += sum;

    return sum;
}

int search_4_delete2(FsNode* node) { // summing all directories on a recursion format and find the minimum directory of required size
    if (node == nullptr)
        return 0;

    int sum = 0;

    for (FsNode* child : node->children) {

        if (child->len != -1)
            sum += child->len;

        sum += search_4_delete2(child);
    }
    if (sum >= required_space && sum < answer2 && sum)
        answer2 = sum;

    return sum;
}

int main() {
    std::list<string> commands;

    ifstream fd("D:\\Coddingg\\C\\AOC\\day7_try2\\input.txt");

    if (!fd.is_open()) {
        cerr << "Error opening the file!";
        return 1;
    }

    string s;                   // reading file
    while (getline(fd, s))
        commands.push_back(s);
    fd.close();

    create_fs(commands); // create filesystem on a general tree form

    search_4_delete(root_node); // sum all firectories under 100000
    cout << "Answer 1: ";
    cout << answer1 << endl;

    sum_fs(root_node); // computed disk in use

    required_space = update_size - (total_disk - disk_in_use);
    search_4_delete2(root_node);
    cout << "Answer 2: ";
    cout << answer2 << endl;
    return 0;
}
