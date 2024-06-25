#include <stdlib.h>
#include <new>

size_t allocated = 0;
void * operator new(size_t size) noexcept(false) {
  allocated += size;
  size_t* sptr = (size_t*)malloc(size + 16);
  *sptr = size;
  char* ptr = (char*)sptr;
  return (void*)(ptr+16);
}

void operator delete(void *ptr) noexcept {
  char* ptr2 = ((char*)ptr) - 16;
  size_t size = *((size_t *)ptr2);
  allocated -= size;

  free((void*)ptr2);
}

#include <cassert>
#include <iostream>
#include <iterator>
#include <re2/re2.h>

int main(int argc, char** argv) {
  bool quiet = false;
  bool utf8 = true;
  for (int i = 1; i < argc; ++i) {
    if (!strcmp(argv[i], "-h") || !strcmp(argv[i], "--help")) {
      std::cerr << R"(Pass a single regex to stdin.

Options:

-h, --help
	this help
-a, --ascii
	latin1 mode instead of utf8
-q, --quiet
	disable reminder to send the regex to stdin
)";
      return 0;
    } else if (!strcmp(argv[i], "-q") || !strcmp(argv[i], "--quiet")) {
      quiet = true;
    } else if (!strcmp(argv[i], "-a") || !strcmp(argv[i], "--ascii")) {
      utf8 = false;
    } else {
      std::cerr << "Unknown option " << argv[i] << std::endl;
      return 0;
    }
  }

  if (!quiet) {
    std::cerr << "Pass a single regex to stdin" << std::endl;
  }
  
  std::istreambuf_iterator<char> begin(std::cin), end;
  std::string re(begin, end);
  
  if (re.empty()) {
    std::cerr << "pass a single regex to stdin" << std::endl;
    return 0;
  }

  size_t before = allocated;

  re2::RE2 r(re, utf8 ? re2::RE2::DefaultOptions : re2::RE2::Latin1);
  assert(r.ok());

  size_t after = allocated;

  int n = r.NumberOfCapturingGroups();
  std::vector<std::string> values;
  values.resize(n);
  std::vector<re2::RE2::Arg> args;
  args.resize(n);
  std::vector<re2::RE2::Arg*> args_refs;
  args_refs.resize(n);

  for (int i = 0; i < n; ++i) {
    args[i] = &values[i];
    args_refs[i] = &args[i];
  }
  re2::RE2::PartialMatchN("this is a test", r, args_refs.data(), n);

  size_t after2 = allocated;

  std::cout << after - before << " " << after2 - after << std::endl;

  return 0;
}
