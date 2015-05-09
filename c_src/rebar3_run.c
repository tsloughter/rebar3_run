/* The MIT License (MIT) */

/* Copyright (c) 2015 Ahmad Sherif */

/* Permission is hereby granted, free of charge, to any person obtaining a copy */
/* of this software and associated documentation files (the "Software"), to deal */
/* in the Software without restriction, including without limitation the rights */
/* to use, copy, modify, merge, publish, distribute, sublicense, and/or sell */
/* copies of the Software, and to permit persons to whom the Software is */
/* furnished to do so, subject to the following conditions: */

/* The above copyright notice and this permission notice shall be included in all */
/* copies or substantial portions of the Software. */

#include <unistd.h>

#include "erl_nif.h"

static ERL_NIF_TERM exec_nif(ErlNifEnv* env, int argc, const ERL_NIF_TERM argv[])
{
  unsigned path_length, args_count, arg_length;
  ERL_NIF_TERM head, tail;
  int i = 0;

  enif_get_list_length(env, argv[0], &path_length);
  enif_get_list_length(env, argv[1], &args_count);

  char* exec_argv[args_count + 2];
  char path[path_length + 1];

  if (!enif_get_string(env, argv[0], path, path_length + 1, ERL_NIF_LATIN1) || !enif_is_list(env, argv[1])) {
    return enif_make_badarg(env);
  }

  tail = argv[1];
  while(enif_get_list_cell(env, tail, &head, &tail) != 0) {
    enif_get_list_length(env, head, &arg_length);

    char* arg = (char*) malloc(sizeof(char) * (arg_length + 1));

    if (!enif_get_string(env, head, arg, arg_length + 1, ERL_NIF_LATIN1)) {
      return enif_make_badarg(env);
    }

    exec_argv[i + 1] = arg;

    i++;
  }

  exec_argv[0] = path;
  exec_argv[args_count + 1] = NULL;

  execv(path, exec_argv);

  return enif_make_atom(env, "ok");
}

static ErlNifFunc nif_funcs[] = {
  {"exec", 2, exec_nif}
};

ERL_NIF_INIT(rebar3_run, nif_funcs, NULL, NULL, NULL, NULL);
