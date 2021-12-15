#include <iostream>

#include <curl/curl.h>

using namespace std;

extern "C" {
	struct CruelRuntime;
	struct CruelString;
	struct CruelStr {
		const uint8_t * data;
		size_t len;
	};
	
	CruelRuntime * cruel_rt_init ();
	void cruel_rt_cleanup (CruelRuntime *);
	CruelString * cruel_rt_resolve (const CruelRuntime *, const CruelStr *);
	
	uint8_t * cruel_string_ptr (const CruelString *);
	size_t cruel_string_len (const CruelString *);
	void cruel_string_free (CruelString *);
}

size_t write_callback (char *ptr, size_t size, size_t nmemb, void *userdata) {
	// Just ignore all the data.
	return size * nmemb;
}

int main () {
	curl_global_init (CURL_GLOBAL_DEFAULT);
	auto rt = cruel_rt_init ();
	
	cout << "Hello from C++!\n";
	
	auto curl = curl_easy_init ();
	
	const string host_str ("example.com");
	const auto url = "https://" + host_str;
	cout << "URL: " << url << '\n';
	curl_easy_setopt (curl, CURLOPT_URL, url.c_str ());
	curl_easy_setopt (curl, CURLOPT_WRITEFUNCTION, write_callback);
	
	curl_slist * slist = nullptr;
	
	{
		// Bunch of dumb FFI stuff needed to make C++ and Rust both pretend
		// they're actually pieces of C code. There is a cleaner way to do
		// this, but it requires more preparation time.
		
		const CruelStr host_str_c {
			// C++ thinks strings are arrays of characters, this is cause C++ is
			// based on C, and C is dumb.
			(const uint8_t *)host_str.data (),
			host_str.size (),
		};
		auto ip_str_c = cruel_rt_resolve (rt, &host_str_c);
		const string ip_str ((const char *)cruel_string_ptr (ip_str_c), cruel_string_len (ip_str_c));
		cruel_string_free (ip_str_c);
		ip_str_c = nullptr;
		
		
		const auto resolve_str = host_str + ":443:" + ip_str;
		cout << "Resolve string: " << resolve_str << '\n';
		slist = curl_slist_append (slist, resolve_str.c_str ());
		
		curl_easy_setopt (curl, CURLOPT_RESOLVE, slist);
	}
	
	auto rc = curl_easy_perform (curl);
	if (rc != CURLE_OK) {
		cout << "Everything is OK.\n";
	}
	else {
		cerr << "Everything is not OK: " << rc << '\n';
	}
	
	curl_slist_free_all (slist);
	slist = nullptr;
	
	curl_easy_cleanup (curl);
	curl = nullptr;
	
	cruel_rt_cleanup (rt);
	curl_global_cleanup();
	
	cout << "Good-bye from C++!\n";
	
	return 0;
}
