/*
Copyright (c) 2012, Broadcom Europe Ltd
All rights reserved.

Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the following conditions are met:
    * Redistributions of source code must retain the above copyright
      notice, this list of conditions and the following disclaimer.
    * Redistributions in binary form must reproduce the above copyright
      notice, this list of conditions and the following disclaimer in the
      documentation and/or other materials provided with the distribution.
    * Neither the name of the copyright holder nor the
      names of its contributors may be used to endorse or promote products
      derived from this software without specific prior written permission.

THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND
ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY
DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
(INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
(INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
*/

// Audio output demo using OpenMAX IL though the ilcient helper library

#include <stdio.h>
#include <math.h>

#include "audio.h"

#define N_WAVE          1024	/* dimension of Sinewave[] */
extern short Sinewave[];

uint8_t
buffer_fill (uint8_t * buf, AUDIOPLAY_CONTEXT_T * context)
{
  context->fills++;

  double TONE_FREQUENCY = 261.6 * 2;	// Hz
  double TONE_PERIOD = 1.0 / TONE_FREQUENCY;	// fraction of a second covered by one tone period.
  double TONE_SAMPLES = TONE_PERIOD * context->samplerate;	// number of samples in one tone period.
  double STEP_SIZE = N_WAVE / TONE_SAMPLES;	// use to step through a periodic function to get the desired tone.

  int16_t *p = (int16_t *) buf;

  // fill the buffer
  for (int i = 0; i < context->buffer_size_samples; i++)
    {
//      int16_t val = Sinewave[(int) context->phase];

      double M = 32767.0 / 2.0;

      int16_t val = (int16_t) M * sin(context->phase / N_WAVE * 2.0 * 3.14159);

      context->phase += STEP_SIZE;
      while (context->phase >= N_WAVE)
	{
	  context->phase -= N_WAVE;
	}

      // fill the channels (mono)
      int j;
      for (j = 0; j < context->channels; j++)
	{
	  if (context->bitdepth == 32)
	    *p++ = 0;
	  *p++ = val;
	}
    }

  int TIME = 10;
  return context->fills <
    ((context->samplerate * TIME) / context->buffer_size_samples);
}

int
main (int argc, char **argv)
{
  bcm_host_init ();

  AUDIOPLAY_CONTEXT_T context;
  init_playback_context (&context);

  if (argc > 1)
    context.audio_dest = atoi (argv[1]);
  if (argc > 2)
    context.channels = atoi (argv[2]);
  if (argc > 3)
    context.samplerate = atoi (argv[3]);

  if (context.audio_dest < 2)
    {
      audio_play (&context, buffer_fill);
    }
  return 0;
}
