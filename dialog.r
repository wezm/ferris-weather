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

resource 'DLOG' (129) {
	{65, 59, 197, 419},
	noGrowDocProc,
	visible,
	goAway,
	0x0,
	130,
	"New Toot",
	staggerMainScreen
};

resource 'DITL' (128, "Temp converter") {
	{	/* array DITLarray: 9 elements */
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
		{74, 88, 94, 311},
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
		},
		/* [9] */
		{70, 50, 102, 82},
		Icon {
			disabled,
			129
		}
	}
};

resource 'DITL' (129, "Error") {
	{	/* array DITLarray: 2 elements */
		/* [1] */
		{70, 190, 90, 248},
		Button {
			enabled,
			"OK"
		},
		/* [2] */
		{10, 70, 59, 251},
		StaticText {
			disabled,
			"Error ^0"
		}
	}
};

resource 'DITL' (130, "Toot") {
	{	/* array DITLarray: 3 elements */
		/* [1] */
		{90, 280, 110, 338},
		Button {
			enabled,
			"Post"
		},
		/* [2] */
		{10, 60, 70, 340},
		EditText {
			enabled,
			"Edit Text"
		},
		/* [3] */
		{10, 10, 42, 42},
		Icon {
			disabled,
			129
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

resource 'ICON' (129, "Ferris") {
	$"0000 0000 0000 0000 0061 8600 00F3 CF00"
	$"01FF FF80 0780 01E0 0E00 0070 1800 0018"
	$"300E 380C 2013 4C04 2013 4C04 301F 7C0C"
	$"780E 381E FC00 007F 737C 3ECE 31C2 C3CC"
	$"1982 8198 0C9E F930 0490 0920 00DE 7B00"
	$"0042 4200 003E 7C"
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

resource 'ALRT' (128) {
	{40, 40, 140, 300},
	129,
	{	/* array: 4 elements */
		/* [1] */
		OK, visible, sound1,
		/* [2] */
		OK, visible, sound1,
		/* [3] */
		OK, visible, sound1,
		/* [4] */
		OK, visible, sound1
	},
	alertPositionMainScreen
};

#include "Menus.r"

resource 'MENU' (128) {
    128, textMenuProc;
    allEnabled, enabled;
    apple;
    {
        "About Toot Classic...", noIcon, noKey, noMark, plain;
        "-", noIcon, noKey, noMark, plain;
    }
};

resource 'MENU' (129) {
    129, textMenuProc;
    allEnabled, enabled;
    "File";
    {
        "New Toot", noIcon, "N", noMark, plain;
        "-", noIcon, noKey, noMark, plain;
        "Close", noIcon, "W", noMark, plain;
        "-", noIcon, noKey, noMark, plain;
        "Quit", noIcon, "Q", noMark, plain;
    }
};

resource 'MENU' (130) {
    130, textMenuProc;
    0, enabled;
    "Edit";
    {
        "Undo", noIcon, "Z", noMark, plain;
        "-", noIcon, noKey, noMark, plain;
        "Cut", noIcon, "X", noMark, plain;
        "Copy", noIcon, "C", noMark, plain;
        "Paste", noIcon, "V", noMark, plain;
        "Clear", noIcon, noKey, noMark, plain;
        "-", noIcon, noKey, noMark, plain;
        "Select All", noIcon, "A", noMark, plain;
    }
};

resource 'MBAR' (128) {
    { 128, 129, 130 };
};
