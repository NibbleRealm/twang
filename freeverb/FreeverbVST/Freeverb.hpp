// Freeverb3 user interface declaration
// Based on Steinberg VST Development Kit Examples
//
// Written by Jezar at Dreampoint, June 2000
// http://www.dreampoint.co.uk
// This code is public domain

#ifndef __Freeverb_H
#define __Freeverb_H

#include "audioeffectx.h"
#include "revmodel.hpp"

enum
{
	KMode, KRoomSize, KDamp, KWidth, KWet, KDry,
	KNumParams
};

class Freeverb : public AudioEffectX
{
public:
					Freeverb(audioMasterCallback audioMaster);
	virtual	void	process(float **inputs, float **outputs, long sampleFrames);
	virtual void	processReplacing(float **inputs, float **outputs, long sampleFrames);
	virtual void	setProgramName(char *name);
	virtual void	getProgramName(char *name);
	virtual void	setParameter(long index, float value);
	virtual float	getParameter(long index);
	virtual void	getParameterLabel(long index, char *label);
	virtual void	getParameterDisplay(long index, char *text);
	virtual void	getParameterName(long index, char *text);
	virtual void	suspend();
	virtual void	resume();
	virtual bool	getEffectName (char* name);
	virtual bool	getVendorString (char* text);
	virtual bool	getProductString (char* text);
	virtual long	canDo(char* text);

private:
	revmodel	model;
	char		programName[32];
};

#endif//_Freeverb_H

//ends

