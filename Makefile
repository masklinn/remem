CXXFLAGS += -std=c++20 -Wall -Werror -g -fPIC -O3
LDFLAGS += -lre2

.PHONY: clean run

run: remem
	@./run < regexen

remem: remem.cpp
	$(CXX) $(CXXFLAGS) $^ -o $@ $(LDFLAGS)

clean:
	@rm remem
