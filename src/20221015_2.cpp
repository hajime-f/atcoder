#include <iostream>
using namespace std;

int main() {
  
  long long X;
  int K;
  
  cin >> X >> K;
  
  double base_number;
  
  for (int i = 0; i < K; i++) {
    base_number = pow(10, i + 1);
    X = round(X / base_number) * base_number;
  }
  
  cout << X << endl;
  
  return 0;
}
