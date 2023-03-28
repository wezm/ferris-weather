/*
	Copyright 2015 Wolfgang Thaller.

	This file is part of Retro68.

	Retro68 is free software: you can redistribute it and/or modify
	it under the terms of the GNU General Public License as published by
	the Free Software Foundation, either version 3 of the License, or
	(at your option) any later version.

	Retro68 is distributed in the hope that it will be useful,
	but WITHOUT ANY WARRANTY; without even the implied warranty of
	MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
	GNU General Public License for more details.

	You should have received a copy of the GNU General Public License
	along with Retro68.  If not, see <http://www.gnu.org/licenses/>.
*/

#include "Dialogs.r"

resource 'DLOG' (128) {
	{50, 100, 240, 420},
	noGrowDocProc,
	visible,
	goAway,
	0x0,
	128,
	"",
	centerMainScreen
};

resource 'DITL' (128) {
	{	/* array DITLarray: 8 elements */
		/* [1] */
		{160, 230, 180, 310},
		Button {
			enabled,
			"Quit"
		},
		/* [2] */
		{155, 225, 185, 315},
		UserItem {
			enabled
		},
		/* [3] */
		{70, 10, 90, 310},
		StaticText {
			enabled,
			"Conversion powered by ^0"
		},
		/* [4] */
		{30, 10, 62, 42},
		Icon {
			disabled,
			128
		},
		/* [5] */
		{20, 50, 36, 125},
		StaticText {
			disabled,
			"Celsius"
		},
		/* [6] */
		{20, 170, 36, 245},
		StaticText {
			disabled,
			"Farenheit"
		},
		/* [7] */
		{41, 54, 57, 129},
		EditText {
			enabled,
			"25"
		},
		/* [8] */
		{43, 174, 59, 249},
		EditText {
			enabled,
			"77"
		}
	}
};

#include "Icons.r"

resource 'ICON' (128) {
        $"0000 0000 0000 E000 0001 1000 0001 1000"
        $"000D 1000 0001 1000 0001 1000 000D 1000"
        $"0001 1000 0001 1000 000D 1000 0001 5000"
        $"0001 5000 000D 5000 0001 5000 0001 5000"
        $"000D 5000 0001 5000 0001 5000 000D 5000"
        $"0001 5000 0001 5000 0001 5000 0001 5000"
        $"0001 5000 0002 4800 0004 4400 0004 E400"
        $"0004 E400 0004 4400 0002 0800 0001 F0"
};

#include "Processes.r"

resource 'SIZE' (-1) {
	reserved,
	acceptSuspendResumeEvents,
	reserved,
	canBackground,
	doesActivateOnFGSwitch,
	backgroundAndForeground,
	dontGetFrontClicks,
	ignoreChildDiedEvents,
	is32BitCompatible,
#ifdef TARGET_API_MAC_CARBON
    isHighLevelEventAware,
#else
	notHighLevelEventAware,
#endif
	onlyLocalHLEvents,
	notStationeryAware,
	dontUseTextEditServices,
	reserved,
	reserved,
	reserved,
#ifdef TARGET_API_MAC_CARBON
	500 * 1024,	// Carbon apparently needs additional memory.
	500 * 1024
#else
	100 * 1024,
	100 * 1024
#endif
};
