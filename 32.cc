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


void solve(){
    int n = 9;
    string nums="123456789";  
    set<int> ans;

    do{
        for(int mul = 1; mul<n; mul++) {
            for(int eq = mul+2; eq<n; eq++) {
                // a*b=c
                string A,B,C;
                for(int i =0; i<mul; i++) A+=nums[i];
                for(int i =mul; i<eq; i++) B+=nums[i];
                for(int i =eq; i<n; i++) C+=nums[i];
                int a = stoi(A);
                int b = stoi(B);
                int c = stoi(C);
                if(a*b==c) {
                    print(a,b,c);
                    ans.insert(c);
                }
            }
        }
    } while(next_permutation(nums.begin(), nums.end()));
    int sum = 0;
    for(auto e: ans) {
        sum += e;
    }
    print(sum);
}
int32_t main() {
    fast;
    solve();
    return 0;
}

