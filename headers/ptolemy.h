/* Generated with cbindgen:0.24.5 */

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct String String;

/**
 *A tuple containing Id 0 the natural language name of the place
 *id 1 the id associated with that name.
 */
typedef struct PlaceNameId {
  struct String name;
  struct String id;
} PlaceNameId;
