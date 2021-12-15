#include <iostream>

using namespace std;

extern "C" {
	void print_hello ();
}

int main () {
	cout << "Hello from C++!\n";
	print_hello ();
	
	return 0;
}
