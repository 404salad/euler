#include <bits/stdc++.h>
using namespace std;
#define fast ios::sync_with_stdio(0);cin.tie(0);
#define rep(i, n)  for (int i = 0; i < (n); i++)
#define rep1(i, n) for (int i = 1; i < (n); i++)
#define repr(i, n) for (int i = (n)-1; i >= 0; i--)
#define all(x) x.begin(), x.end()
#define sum(v) reduce(all(v))
#define pb push_back
#define int long long int
typedef  long double ld;
// 1e7 1e9+7 1e18

int range_sum(int l,int r) {
    if (l > r) return 0;
    return (l + r) * (r - l + 1) / 2;
}
void pv(vector<int> nums) {
    for(auto e: nums) cout << e<<" ";
    cout <<"\n";
}
vector<string> split(const string& str, char delimiter) {
    vector<string> tokens;
    string token;
    istringstream stream(str);
    while (getline(stream, token, delimiter)) {
        tokens.push_back(token);
    }
    return tokens;
}

// Helper function for single argument
template <typename T>
std::string to_string(const T& t) {
    std::ostringstream oss;
    oss << t;
    return oss.str();
}

// Variadic template for multiple arguments
template <typename T, typename... Args>
std::string to_string(const T& first, const Args&... rest) {
    return to_string(first) + ", " + to_string(rest...);
}

// Macro for easy usage
#define print(...) (std::cout << to_string(__VA_ARGS__) << std::endl)

auto is_prime(int n) -> bool{
    for(int i = 2; i*i<=n; i++) {
        if(n%i == 0) return false;
    }
    return n>=2;
}

auto max_primes(int a,int b) -> int {
    int n = 0;
    int x = n*n+a*n+b;
    while(is_prime(x)) {
        //print(x);
        n++;
        x = n*n+a*n+b;
    }
    return n;
}

void solve(){
    int al=-999, ah = 999;
    int bl= -1000,bh =1000;
    int ans = INT_MIN;
    int A = 0,B = 0;
    print(max_primes(1,41));
    for(int a=al; a<=ah; a++) {
        for(int b=bl; b<=bh; b++) {
            int mp = max_primes(a,b);
            if(ans<mp){
                A = a;
                B = b;
                ans = mp;
            }
        }
    }
    print(A*B);
}
 
int32_t main() {
    fast;
    solve();
    return 0;
}

