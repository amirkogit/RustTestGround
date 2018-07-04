#include "test_lib.h"
#include <iostream>

int main(int argc, char* argv[]) 
{
	greetings();
	
	int sum = add(3,5);
	std::cout << "Sum = " << sum << std::endl;
	
	return 0;
}