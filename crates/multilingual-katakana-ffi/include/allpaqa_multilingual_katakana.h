/* C ABI for allpaqa_multilingual_katakana (hand-written; mirrors src/lib.rs).
 * ABI version 1. Rust owns all returned buffers: free with mk_free_string. */
#ifndef ALLPAQA_MULTILINGUAL_KATAKANA_H
#define ALLPAQA_MULTILINGUAL_KATAKANA_H

#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

#define MK_ABI_VERSION 1u

#define MK_OK 0
#define MK_ERR_NULL_POINTER 1
#define MK_ERR_INVALID_UTF8 2
#define MK_ERR_PANIC 3

#define MK_FLAG_ENABLE_CYRILLIC (1u << 0)
#define MK_FLAG_ENABLE_KOREAN (1u << 1)
#define MK_FLAG_ENABLE_CHINESE (1u << 2)
#define MK_FLAG_ENABLE_SPANISH (1u << 3)
#define MK_FLAG_ENABLE_FRENCH (1u << 4)
#define MK_FLAG_ENABLE_VIETNAMESE (1u << 5)
#define MK_FLAG_ENABLE_THAI (1u << 6)
#define MK_FLAG_ENABLE_SLANG (1u << 7)
#define MK_FLAG_ENABLE_ENGLISH (1u << 8)
#define MK_FLAG_NORMALIZE_PROSODY (1u << 9)

/* FROZEN layout: never grows. A flag whose bit is not in flags_set keeps the
 * core default. Unknown bits are ignored. */
typedef struct MkOptions {
  uint32_t flags_set;
  uint32_t flags_value;
} MkOptions;

uint32_t mk_abi_version(void);

/* text: UTF-8, not NUL-terminated; text_ptr may be NULL only if text_len == 0.
 * options may be NULL (all defaults). On success *out_ptr and *out_len hold a
 * Rust-allocated buffer (empty output = non-NULL pointer, length 0); release
 * with mk_free_string. On error *out_ptr = NULL and *out_len = 0. */
int32_t mk_to_katakana(const uint8_t *text_ptr, size_t text_len,
                       const MkOptions *options, uint8_t **out_ptr,
                       size_t *out_len);

/* NULL (or len == 0) is a no-op. */
void mk_free_string(uint8_t *ptr, size_t len);

/* Returns 1 if the native self check passes, else 0. */
int32_t mk_self_check(void);

#ifdef __cplusplus
}
#endif

#endif /* ALLPAQA_MULTILINGUAL_KATAKANA_H */
