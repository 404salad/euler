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

template<typename T> struct is_pair : false_type {}; template<typename T, typename U> struct is_pair<pair<T, U>> : true_type {}; template<typename T> struct is_string : false_type {}; template<> struct is_string<string> : true_type {}; template<> struct is_string<const char*> : true_type {}; template<> struct is_string<char*> : true_type {}; template<typename T, typename = void> struct is_container : false_type {}; template<typename T> struct is_container<T, void_t<decltype(declval<T>().begin()), decltype(declval<T>().end())>> : bool_constant<!is_string<T>::value> {}; template<typename T> void print(const T& x) { if constexpr (is_pair<T>::value) { cout << "("; print(x.first); cout << ", "; print(x.second); cout << ")"; } else if constexpr (is_string<T>::value) { cout << "\"" << x << "\""; } else if constexpr (is_container<T>::value) { cout << "["; bool first = true; for (const auto& elem : x) { if (!first) cout << ", "; print(elem); first = false; } cout << "]"; } else { cout << x; } }

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

void solve(){
    map<int,int> cache;
    auto fn = [&](int i) {
        if(cache.count(i)) {
            return cache[i];
        }
        else {
            return cache[i] = (i*(3*i-1))/2;
        }
    };
    auto isp = [](int i) {
        auto okay = [](double x) {
            int a = (int)x;
            if((double)a == x) return true;
            return false;
        };

        auto e =sqrt(24*i+1);
        if(okay(e) && ((int)e)%6 == 5){
            return true;
        }
        return false;
    };


    int n = 10000;
    for(int i =1; i<n; i++) {
        int a = fn(i);
        for(int j =2; j<n; j++) {
            int b = fn(j);
            if(isp(b-a) && isp(a+b)){
                cout << "|"<<b-a <<"|\n";
                return;
            }
        }
    }
}

int32_t main() {
    fast;
    solve();
    return 0;
}

