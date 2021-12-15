#include <iostream>

using namespace std;

extern "C" {
	struct CruelRuntime;
	
	CruelRuntime * cruel_rt_init ();
	void cruel_rt_cleanup (CruelRuntime *);
	void cruel_rt_hello (const CruelRuntime *);
}

int main () {
	cout << "Hello from C++!\n";
	
	auto rt = cruel_rt_init ();
	
	cruel_rt_hello (rt);
	
	cruel_rt_cleanup (rt);
	
	cout << "Good-bye from C++!\n";
	
	return 0;
}
