use mouse_keyboard_input::*;

pub fn match_str(str: &str) -> Option<u16> {
    match str {
        "INPUT_PROP_POINTER" => Some(0x00),        /* needs a pointer */
        "INPUT_PROP_DIRECT" => Some(0x01),         /* direct input devices */
        "INPUT_PROP_BUTTONPAD" => Some(0x02),      /* has button" => Some(s under pad */
        "INPUT_PROP_SEMI_MT" => Some(0x03),        /* touch rectangle only */
        "INPUT_PROP_TOPBUTTONPAD" => Some(0x04),   /* softbuttons at top of pad */
        "INPUT_PROP_POINTING_STICK" => Some(0x05), /* is a pointing stick */
        "INPUT_PROP_ACCELEROMETER" => Some(0x06),  /* has accelerometer */

        "INPUT_PROP_MAX" => Some(0x1f),
        "INPUT_PROP_CNT" => Some(INPUT_PROP_MAX + 1),

        /*
         * Event types
         */
        "EV_SYN" => Some(0x00),
        "EV_KEY" => Some(0x01),
        "EV_REL" => Some(0x02),
        "EV_ABS" => Some(0x03),
        "EV_MSC" => Some(0x04),
        "EV_SW" => Some(0x05),
        "EV_LED" => Some(0x11),
        "EV_SND" => Some(0x12),
        "EV_REP" => Some(0x14),
        "EV_FF" => Some(0x15),
        "EV_PWR" => Some(0x16),
        "EV_FF_STATUS" => Some(0x17),
        "EV_MAX" => Some(0x1f),
        "EV_CNT" => Some(EV_MAX + 1),

        /*
         * Synchronization events.
         */
        "SYN_REPORT" => Some(0),
        "SYN_CONFIG" => Some(1),
        "SYN_MT_REPORT" => Some(2),
        "SYN_DROPPED" => Some(3),
        "SYN_MAX" => Some(0xf),
        "SYN_CNT" => Some(SYN_MAX + 1),

        /*
         * Keys and buttons
         *
         * Most of the keys/buttons are modeled after USB HUT 1.12 (see
         * http://www.usb.org/developers/hidpage).
         * Abbreviations in the comments:
         * AC - Application Control
         * AL - Application Launch Button
         * SC - System Control
         */
        "KEY_RESERVED" => Some(0),
        "KEY_ESC" => Some(1),
        "KEY_1" => Some(2),
        "KEY_2" => Some(3),
        "KEY_3" => Some(4),
        "KEY_4" => Some(5),
        "KEY_5" => Some(6),
        "KEY_6" => Some(7),
        "KEY_7" => Some(8),
        "KEY_8" => Some(9),
        "KEY_9" => Some(10),
        "KEY_10" => Some(11),
        "KEY_MINUS" => Some(12),
        "KEY_EQUAL" => Some(13),
        "KEY_BACKSPACE" => Some(14),
        "KEY_TAB" => Some(15),
        "KEY_Q" => Some(16),
        "KEY_W" => Some(17),
        "KEY_E" => Some(18),
        "KEY_R" => Some(19),
        "KEY_T" => Some(20),
        "KEY_Y" => Some(21),
        "KEY_U" => Some(22),
        "KEY_I" => Some(23),
        "KEY_O" => Some(24),
        "KEY_P" => Some(25),
        "KEY_LEFTBRACE" => Some(26),
        "KEY_RIGHTBRACE" => Some(27),
        "KEY_ENTER" => Some(28),
        "KEY_LEFTCTRL" => Some(29),
        "KEY_A" => Some(30),
        "KEY_S" => Some(31),
        "KEY_D" => Some(32),
        "KEY_F" => Some(33),
        "KEY_G" => Some(34),
        "KEY_H" => Some(35),
        "KEY_J" => Some(36),
        "KEY_K" => Some(37),
        "KEY_L" => Some(38),
        "KEY_SEMICOLON" => Some(39),
        "KEY_APOSTROPHE" => Some(40),
        "KEY_GRAVE" => Some(41),
        "KEY_LEFTSHIFT" => Some(42),
        "KEY_BACKSLASH" => Some(43),
        "KEY_Z" => Some(44),
        "KEY_X" => Some(45),
        "KEY_C" => Some(46),
        "KEY_V" => Some(47),
        "KEY_B" => Some(48),
        "KEY_N" => Some(49),
        "KEY_M" => Some(50),
        "KEY_COMMA" => Some(51),
        "KEY_DOT" => Some(52),
        "KEY_SLASH" => Some(53),
        "KEY_RIGHTSHIFT" => Some(54),
        "KEY_KPASTERISK" => Some(55),
        "KEY_LEFTALT" => Some(56),
        "KEY_SPACE" => Some(57),
        "KEY_CAPSLOCK" => Some(58),
        "KEY_F1" => Some(59),
        "KEY_F2" => Some(60),
        "KEY_F3" => Some(61),
        "KEY_F4" => Some(62),
        "KEY_F5" => Some(63),
        "KEY_F6" => Some(64),
        "KEY_F7" => Some(65),
        "KEY_F8" => Some(66),
        "KEY_F9" => Some(67),
        "KEY_F10" => Some(68),
        "KEY_NUMLOCK" => Some(69),
        "KEY_SCROLLLOCK" => Some(70),
        "KEY_KP7" => Some(71),
        "KEY_KP8" => Some(72),
        "KEY_KP9" => Some(73),
        "KEY_KPMINUS" => Some(74),
        "KEY_KP4" => Some(75),
        "KEY_KP5" => Some(76),
        "KEY_KP6" => Some(77),
        "KEY_KPPLUS" => Some(78),
        "KEY_KP1" => Some(79),
        "KEY_KP2" => Some(80),
        "KEY_KP3" => Some(81),
        "KEY_KP0" => Some(82),
        "KEY_KPDOT" => Some(83),

        "KEY_ZENKAKUHANKAKU" => Some(85),
        "KEY_102ND" => Some(86),
        "KEY_F11" => Some(87),
        "KEY_F12" => Some(88),
        "KEY_RO" => Some(89),
        "KEY_KATAKANA" => Some(90),
        "KEY_HIRAGANA" => Some(91),
        "KEY_HENKAN" => Some(92),
        "KEY_KATAKANAHIRAGANA" => Some(93),
        "KEY_MUHENKAN" => Some(94),
        "KEY_KPJPCOMMA" => Some(95),
        "KEY_KPENTER" => Some(96),
        "KEY_RIGHTCTRL" => Some(97),
        "KEY_KPSLASH" => Some(98),
        "KEY_SYSRQ" => Some(99),
        "KEY_RIGHTALT" => Some(100),
        "KEY_LINEFEED" => Some(101),
        "KEY_HOME" => Some(102),
        "KEY_UP" => Some(103),
        "KEY_PAGEUP" => Some(104),
        "KEY_LEFT" => Some(105),
        "KEY_RIGHT" => Some(106),
        "KEY_END" => Some(107),
        "KEY_DOWN" => Some(108),
        "KEY_PAGEDOWN" => Some(109),
        "KEY_INSERT" => Some(110),
        "KEY_DELETE" => Some(111),
        "KEY_MACRO" => Some(112),
        "KEY_MUTE" => Some(113),
        "KEY_VOLUMEDOWN" => Some(114),
        "KEY_VOLUMEUP" => Some(115),
        "KEY_POWER" => Some(116), /* SC System Power Down */
        "KEY_KPEQUAL" => Some(117),
        "KEY_KPPLUSMINUS" => Some(118),
        "KEY_PAUSE" => Some(119),
        "KEY_SCALE" => Some(120), /* AL Compiz Scale " => Some(Expose */

        "KEY_KPCOMMA" => Some(121),
        "KEY_HANGEUL" => Some(122),
        "KEY_HANGUEL" => Some(KEY_HANGEUL),
        "KEY_HANJA" => Some(123),
        "KEY_YEN" => Some(124),
        "KEY_LEFTMETA" => Some(125),
        "KEY_RIGHTMETA" => Some(126),
        "KEY_COMPOSE" => Some(127),

        "KEY_STOP" => Some(128), /* AC Stop */
        "KEY_AGAIN" => Some(129),
        "KEY_PROPS" => Some(130), /* AC Properties */
        "KEY_UNDO" => Some(131),  /* AC Undo */
        "KEY_FRONT" => Some(132),
        "KEY_COPY" => Some(133),  /* AC Copy */
        "KEY_OPEN" => Some(134),  /* AC Open */
        "KEY_PASTE" => Some(135), /* AC Paste */
        "KEY_FIND" => Some(136),  /* AC Search */
        "KEY_CUT" => Some(137),   /* AC Cut */
        "KEY_HELP" => Some(138),  /* AL Integrated Help Center */
        "KEY_MENU" => Some(139),  /* Menu " => Some(show menu */
        "KEY_CALC" => Some(140),  /* AL Calculator */
        "KEY_SETUP" => Some(141),
        "KEY_SLEEP" => Some(142),  /* SC System Sleep */
        "KEY_WAKEUP" => Some(143), /* System Wake Up */
        "KEY_FILE" => Some(144),   /* AL Local Machine Browser */
        "KEY_SENDFILE" => Some(145),
        "KEY_DELETEFILE" => Some(146),
        "KEY_XFER" => Some(147),
        "KEY_PROG1" => Some(148),
        "KEY_PROG2" => Some(149),
        "KEY_WWW" => Some(150), /* AL Internet Browser */
        "KEY_MSDOS" => Some(151),
        "KEY_COFFEE" => Some(152), /* AL Terminal Lock/Screensaver */
        "KEY_SCREENLOCK" => Some(KEY_COFFEE),
        "KEY_ROTATE_DISPLAY" => Some(153), /* Display orientation for e.g. tablets */
        "KEY_DIRECTION" => Some(KEY_ROTATE_DISPLAY),
        "KEY_CYCLEWINDOWS" => Some(154),
        "KEY_MAIL" => Some(155),
        "KEY_BOOKMARKS" => Some(156), /* AC Bookmarks */
        "KEY_COMPUTER" => Some(157),
        "KEY_BACK" => Some(158),    /* AC Back */
        "KEY_FORWARD" => Some(159), /* AC Forward */
        "KEY_CLOSECD" => Some(160),
        "KEY_EJECTCD" => Some(161),
        "KEY_EJECTCLOSECD" => Some(162),
        "KEY_NEXTSONG" => Some(163),
        "KEY_PLAYPAUSE" => Some(164),
        "KEY_PREVIOUSSONG" => Some(165),
        "KEY_STOPCD" => Some(166),
        "KEY_RECORD" => Some(167),
        "KEY_REWIND" => Some(168),
        "KEY_PHONE" => Some(169), /* Media Select Telephone */
        "KEY_ISO" => Some(170),
        "KEY_CONFIG" => Some(171), /* AL Consumer Control Configuration */
        "KEY_HOMEPAGE" => Some(172), /* AC Home */
        "KEY_REFRESH" => Some(173), /* AC Refresh */
        "KEY_EXIT" => Some(174),   /* AC Exit */
        "KEY_MOVE" => Some(175),
        "KEY_EDIT" => Some(176),
        "KEY_SCROLLUP" => Some(177),
        "KEY_SCROLLDOWN" => Some(178),
        "KEY_KPLEFTPAREN" => Some(179),
        "KEY_KPRIGHTPAREN" => Some(180),
        "KEY_NEW" => Some(181),  /* AC New */
        "KEY_REDO" => Some(182), /* AC Redo/Repeat */

        "KEY_F13" => Some(183),
        "KEY_F14" => Some(184),
        "KEY_F15" => Some(185),
        "KEY_F16" => Some(186),
        "KEY_F17" => Some(187),
        "KEY_F18" => Some(188),
        "KEY_F19" => Some(189),
        "KEY_F20" => Some(190),
        "KEY_F21" => Some(191),
        "KEY_F22" => Some(192),
        "KEY_F23" => Some(193),
        "KEY_F24" => Some(194),

        "KEY_PLAYCD" => Some(200),
        "KEY_PAUSECD" => Some(201),
        "KEY_PROG3" => Some(202),
        "KEY_PROG4" => Some(203),
        "KEY_DASHBOARD" => Some(204), /* AL Dashboard */
        "KEY_SUSPEND" => Some(205),
        "KEY_CLOSE" => Some(206), /* AC Close */
        "KEY_PLAY" => Some(207),
        "KEY_FASTFORWARD" => Some(208),
        "KEY_BASSBOOST" => Some(209),
        "KEY_PRINT" => Some(210), /* AC Print */
        "KEY_HP" => Some(211),
        "KEY_CAMERA" => Some(212),
        "KEY_SOUND" => Some(213),
        "KEY_QUESTION" => Some(214),
        "KEY_EMAIL" => Some(215),
        "KEY_CHAT" => Some(216),
        "KEY_SEARCH" => Some(217),
        "KEY_CONNECT" => Some(218),
        "KEY_FINANCE" => Some(219), /* AL Checkbook/Finance */
        "KEY_SPORT" => Some(220),
        "KEY_SHOP" => Some(221),
        "KEY_ALTERASE" => Some(222),
        "KEY_CANCEL" => Some(223), /* AC Cancel */
        "KEY_BRIGHTNESSDOWN" => Some(224),
        "KEY_BRIGHTNESSUP" => Some(225),
        "KEY_MEDIA" => Some(226),

        "KEY_SWITCHVIDEOMODE" => Some(227), /* Cycle between available video outputs (Monitor/LCD/TV-out/etc) */
        "KEY_KBDILLUMTOGGLE" => Some(228),
        "KEY_KBDILLUMDOWN" => Some(229),
        "KEY_KBDILLUMUP" => Some(230),

        "KEY_SEND" => Some(231),        /* AC Send */
        "KEY_REPLY" => Some(232),       /* AC Reply */
        "KEY_FORWARDMAIL" => Some(233), /* AC Forward Msg */
        "KEY_SAVE" => Some(234),        /* AC Save */
        "KEY_DOCUMENTS" => Some(235),

        "KEY_BATTERY" => Some(236),

        "KEY_BLUETOOTH" => Some(237),
        "KEY_WLAN" => Some(238),
        "KEY_UWB" => Some(239),

        "KEY_UNKNOWN" => Some(240),

        "KEY_VIDEO_NEXT" => Some(241), /* drive next video source */
        "KEY_VIDEO_PREV" => Some(242), /* drive previous video source */
        "KEY_BRIGHTNESS_CYCLE" => Some(243), /* brightness up), after max is min */
        "KEY_BRIGHTNESS_AUTO" => Some(244), /* Set Auto Brightness" =>manual brightness control is off), rely on ambient */
        "KEY_BRIGHTNESS_ZERO" => Some(KEY_BRIGHTNESS_AUTO),
        "KEY_DISPLAY_OFF" => Some(245), /* display device to off state */

        "KEY_WWAN" => Some(246), /* Wireless WAN " => Some(LTE), UMTS), GSM), etc. */
        "KEY_WIMAX" => Some(KEY_WWAN),
        "KEY_RFKILL" => Some(247), /* Key that controls all radios */

        "KEY_MICMUTE" => Some(248), /* Mute / unmute the microphone */

        /* Code 255 is reserved for special needs of AT keyboard driver */
        "BTN_MISC" => Some(0x100),
        "BTN_0" => Some(0x100),
        "BTN_1" => Some(0x101),
        "BTN_2" => Some(0x102),
        "BTN_3" => Some(0x103),
        "BTN_4" => Some(0x104),
        "BTN_5" => Some(0x105),
        "BTN_6" => Some(0x106),
        "BTN_7" => Some(0x107),
        "BTN_8" => Some(0x108),
        "BTN_9" => Some(0x109),

        "BTN_MOUSE" => Some(0x110),
        "BTN_LEFT" => Some(0x110),
        "BTN_RIGHT" => Some(0x111),
        "BTN_MIDDLE" => Some(0x112),
        "BTN_SIDE" => Some(0x113),
        "BTN_EXTRA" => Some(0x114),
        "BTN_FORWARD" => Some(0x115),
        "BTN_BACK" => Some(0x116),
        "BTN_TASK" => Some(0x117),

        "BTN_JOYSTICK" => Some(0x120),
        "BTN_TRIGGER" => Some(0x120),
        "BTN_THUMB" => Some(0x121),
        "BTN_THUMB2" => Some(0x122),
        "BTN_TOP" => Some(0x123),
        "BTN_TOP2" => Some(0x124),
        "BTN_PINKIE" => Some(0x125),
        "BTN_BASE" => Some(0x126),
        "BTN_BASE2" => Some(0x127),
        "BTN_BASE3" => Some(0x128),
        "BTN_BASE4" => Some(0x129),
        "BTN_BASE5" => Some(0x12a),
        "BTN_BASE6" => Some(0x12b),
        "BTN_DEAD" => Some(0x12f),

        "BTN_GAMEPAD" => Some(0x130),
        "BTN_SOUTH" => Some(0x130),
        "BTN_A" => Some(BTN_SOUTH),
        "BTN_EAST" => Some(0x131),
        "BTN_B" => Some(BTN_EAST),
        "BTN_C" => Some(0x132),
        "BTN_NORTH" => Some(0x133),
        "BTN_X" => Some(BTN_NORTH),
        "BTN_WEST" => Some(0x134),
        "BTN_Y" => Some(BTN_WEST),
        "BTN_Z" => Some(0x135),
        "BTN_TL" => Some(0x136),
        "BTN_TR" => Some(0x137),
        "BTN_TL2" => Some(0x138),
        "BTN_TR2" => Some(0x139),
        "BTN_SELECT" => Some(0x13a),
        "BTN_START" => Some(0x13b),
        "BTN_MODE" => Some(0x13c),
        "BTN_THUMBL" => Some(0x13d),
        "BTN_THUMBR" => Some(0x13e),

        "BTN_DIGI" => Some(0x140),
        "BTN_TOOL_PEN" => Some(0x140),
        "BTN_TOOL_RUBBER" => Some(0x141),
        "BTN_TOOL_BRUSH" => Some(0x142),
        "BTN_TOOL_PENCIL" => Some(0x143),
        "BTN_TOOL_AIRBRUSH" => Some(0x144),
        "BTN_TOOL_FINGER" => Some(0x145),
        "BTN_TOOL_MOUSE" => Some(0x146),
        "BTN_TOOL_LENS" => Some(0x147),
        "BTN_TOOL_QUINTTAP" => Some(0x148), /* Five fingers on trackpad */
        "BTN_TOUCH" => Some(0x14a),
        "BTN_STYLUS" => Some(0x14b),
        "BTN_STYLUS2" => Some(0x14c),
        "BTN_TOOL_DOUBLETAP" => Some(0x14d),
        "BTN_TOOL_TRIPLETAP" => Some(0x14e),
        "BTN_TOOL_QUADTAP" => Some(0x14f), /* Four fingers on trackpad */

        "BTN_WHEEL" => Some(0x150),
        "BTN_GEAR_DOWN" => Some(0x150),
        "BTN_GEAR_UP" => Some(0x151),

        "KEY_OK" => Some(0x160),
        "KEY_SELECT" => Some(0x161),
        "KEY_GOTO" => Some(0x162),
        "KEY_CLEAR" => Some(0x163),
        "KEY_POWER2" => Some(0x164),
        "KEY_OPTION" => Some(0x165),
        "KEY_INFO" => Some(0x166), /* AL OEM Features/Tips/Tutorial */
        "KEY_TIME" => Some(0x167),
        "KEY_VENDOR" => Some(0x168),
        "KEY_ARCHIVE" => Some(0x169),
        "KEY_PROGRAM" => Some(0x16a), /* Media Select Program Guide */
        "KEY_CHANNEL" => Some(0x16b),
        "KEY_FAVORITES" => Some(0x16c),
        "KEY_EPG" => Some(0x16d),
        "KEY_PVR" => Some(0x16e), /* Media Select Home */
        "KEY_MHP" => Some(0x16f),
        "KEY_LANGUAGE" => Some(0x170),
        "KEY_TITLE" => Some(0x171),
        "KEY_SUBTITLE" => Some(0x172),
        "KEY_ANGLE" => Some(0x173),
        "KEY_ZOOM" => Some(0x174),
        "KEY_MODE" => Some(0x175),
        "KEY_KEYBOARD" => Some(0x176),
        "KEY_SCREEN" => Some(0x177),
        "KEY_PC" => Some(0x178),   /* Media Select Computer */
        "KEY_TV" => Some(0x179),   /* Media Select TV */
        "KEY_TV2" => Some(0x17a),  /* Media Select Cable */
        "KEY_VCR" => Some(0x17b),  /* Media Select VCR */
        "KEY_VCR2" => Some(0x17c), /* VCR Plus */
        "KEY_SAT" => Some(0x17d),  /* Media Select Satellite */
        "KEY_SAT2" => Some(0x17e),
        "KEY_CD" => Some(0x17f),   /* Media Select CD */
        "KEY_TAPE" => Some(0x180), /* Media Select Tape */
        "KEY_RADIO" => Some(0x181),
        "KEY_TUNER" => Some(0x182), /* Media Select Tuner */
        "KEY_PLAYER" => Some(0x183),
        "KEY_TEXT" => Some(0x184),
        "KEY_DVD" => Some(0x185), /* Media Select DVD */
        "KEY_AUX" => Some(0x186),
        "KEY_MP3" => Some(0x187),
        "KEY_AUDIO" => Some(0x188), /* AL Audio Browser */
        "KEY_VIDEO" => Some(0x189), /* AL Movie Browser */
        "KEY_DIRECTORY" => Some(0x18a),
        "KEY_LIST" => Some(0x18b),
        "KEY_MEMO" => Some(0x18c), /* Media Select Messages */
        "KEY_CALENDAR" => Some(0x18d),
        "KEY_RED" => Some(0x18e),
        "KEY_GREEN" => Some(0x18f),
        "KEY_YELLOW" => Some(0x190),
        "KEY_BLUE" => Some(0x191),
        "KEY_CHANNELUP" => Some(0x192),   /* Channel Increment */
        "KEY_CHANNELDOWN" => Some(0x193), /* Channel Decrement */
        "KEY_FIRST" => Some(0x194),
        "KEY_LAST" => Some(0x195), /* Recall Last */
        "KEY_AB" => Some(0x196),
        "KEY_NEXT" => Some(0x197),
        "KEY_RESTART" => Some(0x198),
        "KEY_SLOW" => Some(0x199),
        "KEY_SHUFFLE" => Some(0x19a),
        "KEY_BREAK" => Some(0x19b),
        "KEY_PREVIOUS" => Some(0x19c),
        "KEY_DIGITS" => Some(0x19d),
        "KEY_TEEN" => Some(0x19e),
        "KEY_TWEN" => Some(0x19f),
        "KEY_VIDEOPHONE" => Some(0x1a0), /* Media Select Video Phone */
        "KEY_GAMES" => Some(0x1a1),      /* Media Select Games */
        "KEY_ZOOMIN" => Some(0x1a2),     /* AC Zoom In */
        "KEY_ZOOMOUT" => Some(0x1a3),    /* AC Zoom Out */
        "KEY_ZOOMRESET" => Some(0x1a4),  /* AC Zoom */
        "KEY_WORDPROCESSOR" => Some(0x1a5), /* AL Word Processor */
        "KEY_EDITOR" => Some(0x1a6),     /* AL Text Editor */
        "KEY_SPREADSHEET" => Some(0x1a7), /* AL Spreadsheet */
        "KEY_GRAPHICSEDITOR" => Some(0x1a8), /* AL Graphics Editor */
        "KEY_PRESENTATION" => Some(0x1a9), /* AL Presentation App */
        "KEY_DATABASE" => Some(0x1aa),   /* AL Database App */
        "KEY_NEWS" => Some(0x1ab),       /* AL Newsreader */
        "KEY_VOICEMAIL" => Some(0x1ac),  /* AL Voicemail */
        "KEY_ADDRESSBOOK" => Some(0x1ad), /* AL Contacts/Address Book */
        "KEY_MESSENGER" => Some(0x1ae),  /* AL Instant Messaging */
        "KEY_DISPLAYTOGGLE" => Some(0x1af), /* Turn display " => Some(LCD on and off */
        "KEY_BRIGHTNESS_TOGGLE" => Some(KEY_DISPLAYTOGGLE),
        "KEY_SPELLCHECK" => Some(0x1b0), /* AL Spell Check */
        "KEY_LOGOFF" => Some(0x1b1),     /* AL Logoff */

        "KEY_DOLLAR" => Some(0x1b2),
        "KEY_EURO" => Some(0x1b3),

        "KEY_FRAMEBACK" => Some(0x1b4), /* Consumer - transport controls */
        "KEY_FRAMEFORWARD" => Some(0x1b5),
        "KEY_CONTEXT_MENU" => Some(0x1b6), /* GenDesc - system context menu */
        "KEY_MEDIA_REPEAT" => Some(0x1b7), /* Consumer - transport control */
        "KEY_10CHANNELSUP" => Some(0x1b8), /* 10 channels up " => Some(10+ */
        "KEY_10CHANNELSDOWN" => Some(0x1b9), /* 10 channels down " => Some(10- */
        "KEY_IMAGES" => Some(0x1ba),       /* AL Image Browser */

        "KEY_DEL_EOL" => Some(0x1c0),
        "KEY_DEL_EOS" => Some(0x1c1),
        "KEY_INS_LINE" => Some(0x1c2),
        "KEY_DEL_LINE" => Some(0x1c3),

        "KEY_FN" => Some(0x1d0),
        "KEY_FN_ESC" => Some(0x1d1),
        "KEY_FN_F1" => Some(0x1d2),
        "KEY_FN_F2" => Some(0x1d3),
        "KEY_FN_F3" => Some(0x1d4),
        "KEY_FN_F4" => Some(0x1d5),
        "KEY_FN_F5" => Some(0x1d6),
        "KEY_FN_F6" => Some(0x1d7),
        "KEY_FN_F7" => Some(0x1d8),
        "KEY_FN_F8" => Some(0x1d9),
        "KEY_FN_F9" => Some(0x1da),
        "KEY_FN_F10" => Some(0x1db),
        "KEY_FN_F11" => Some(0x1dc),
        "KEY_FN_F12" => Some(0x1dd),
        "KEY_FN_1" => Some(0x1de),
        "KEY_FN_2" => Some(0x1df),
        "KEY_FN_D" => Some(0x1e0),
        "KEY_FN_E" => Some(0x1e1),
        "KEY_FN_F" => Some(0x1e2),
        "KEY_FN_S" => Some(0x1e3),
        "KEY_FN_B" => Some(0x1e4),

        "KEY_BRL_DOT1" => Some(0x1f1),
        "KEY_BRL_DOT2" => Some(0x1f2),
        "KEY_BRL_DOT3" => Some(0x1f3),
        "KEY_BRL_DOT4" => Some(0x1f4),
        "KEY_BRL_DOT5" => Some(0x1f5),
        "KEY_BRL_DOT6" => Some(0x1f6),
        "KEY_BRL_DOT7" => Some(0x1f7),
        "KEY_BRL_DOT8" => Some(0x1f8),
        "KEY_BRL_DOT9" => Some(0x1f9),
        "KEY_BRL_DOT10" => Some(0x1fa),

        "KEY_NUMERIC_0" => Some(0x200), /* used by phones), remote controls), */
        "KEY_NUMERIC_1" => Some(0x201), /* and other keypads */
        "KEY_NUMERIC_2" => Some(0x202),
        "KEY_NUMERIC_3" => Some(0x203),
        "KEY_NUMERIC_4" => Some(0x204),
        "KEY_NUMERIC_5" => Some(0x205),
        "KEY_NUMERIC_6" => Some(0x206),
        "KEY_NUMERIC_7" => Some(0x207),
        "KEY_NUMERIC_8" => Some(0x208),
        "KEY_NUMERIC_9" => Some(0x209),
        "KEY_NUMERIC_STAR" => Some(0x20a),
        "KEY_NUMERIC_POUND" => Some(0x20b),
        "KEY_NUMERIC_A" => Some(0x20c), /* Phone key A - HUT Telephony 0xb9 */
        "KEY_NUMERIC_B" => Some(0x20d),
        "KEY_NUMERIC_C" => Some(0x20e),
        "KEY_NUMERIC_D" => Some(0x20f),

        "KEY_CAMERA_FOCUS" => Some(0x210),
        "KEY_WPS_BUTTON" => Some(0x211), /* WiFi Protected Setup key */

        "KEY_TOUCHPAD_TOGGLE" => Some(0x212), /* Request switch touchpad on or off */
        "KEY_TOUCHPAD_ON" => Some(0x213),
        "KEY_TOUCHPAD_OFF" => Some(0x214),

        "KEY_CAMERA_ZOOMIN" => Some(0x215),
        "KEY_CAMERA_ZOOMOUT" => Some(0x216),
        "KEY_CAMERA_UP" => Some(0x217),
        "KEY_CAMERA_DOWN" => Some(0x218),
        "KEY_CAMERA_LEFT" => Some(0x219),
        "KEY_CAMERA_RIGHT" => Some(0x21a),

        "KEY_ATTENDANT_ON" => Some(0x21b),
        "KEY_ATTENDANT_OFF" => Some(0x21c),
        "KEY_ATTENDANT_TOGGLE" => Some(0x21d), /* Attendant call on or off */
        "KEY_LIGHTS_TOGGLE" => Some(0x21e),    /* Reading light on or off */

        "BTN_DPAD_UP" => Some(0x220),
        "BTN_DPAD_DOWN" => Some(0x221),
        "BTN_DPAD_LEFT" => Some(0x222),
        "BTN_DPAD_RIGHT" => Some(0x223),

        "KEY_ALS_TOGGLE" => Some(0x230), /* Ambient light sensor */

        "KEY_BUTTONCONFIG" => Some(0x240), /* AL Button Configuration */
        "KEY_TASKMANAGER" => Some(0x241),  /* AL Task/Project Manager */
        "KEY_JOURNAL" => Some(0x242),      /* AL Log/Journal/Timecard */
        "KEY_CONTROLPANEL" => Some(0x243), /* AL Control Panel */
        "KEY_APPSELECT" => Some(0x244),    /* AL Select Task/Application */
        "KEY_SCREENSAVER" => Some(0x245),  /* AL Screen Saver */
        "KEY_VOICECOMMAND" => Some(0x246), /* Listening Voice Command */

        "KEY_BRIGHTNESS_MIN" => Some(0x250), /* Set Brightness to Minimum */
        "KEY_BRIGHTNESS_MAX" => Some(0x251), /* Set Brightness to Maximum */

        "KEY_KBDINPUTASSIST_PREV" => Some(0x260),
        "KEY_KBDINPUTASSIST_NEXT" => Some(0x261),
        "KEY_KBDINPUTASSIST_PREVGROUP" => Some(0x262),
        "KEY_KBDINPUTASSIST_NEXTGROUP" => Some(0x263),
        "KEY_KBDINPUTASSIST_ACCEPT" => Some(0x264),
        "KEY_KBDINPUTASSIST_CANCEL" => Some(0x265),

        "BTN_TRIGGER_HAPPY" => Some(0x2c0),
        "BTN_TRIGGER_HAPPY1" => Some(0x2c0),
        "BTN_TRIGGER_HAPPY2" => Some(0x2c1),
        "BTN_TRIGGER_HAPPY3" => Some(0x2c2),
        "BTN_TRIGGER_HAPPY4" => Some(0x2c3),
        "BTN_TRIGGER_HAPPY5" => Some(0x2c4),
        "BTN_TRIGGER_HAPPY6" => Some(0x2c5),
        "BTN_TRIGGER_HAPPY7" => Some(0x2c6),
        "BTN_TRIGGER_HAPPY8" => Some(0x2c7),
        "BTN_TRIGGER_HAPPY9" => Some(0x2c8),
        "BTN_TRIGGER_HAPPY10" => Some(0x2c9),
        "BTN_TRIGGER_HAPPY11" => Some(0x2ca),
        "BTN_TRIGGER_HAPPY12" => Some(0x2cb),
        "BTN_TRIGGER_HAPPY13" => Some(0x2cc),
        "BTN_TRIGGER_HAPPY14" => Some(0x2cd),
        "BTN_TRIGGER_HAPPY15" => Some(0x2ce),
        "BTN_TRIGGER_HAPPY16" => Some(0x2cf),
        "BTN_TRIGGER_HAPPY17" => Some(0x2d0),
        "BTN_TRIGGER_HAPPY18" => Some(0x2d1),
        "BTN_TRIGGER_HAPPY19" => Some(0x2d2),
        "BTN_TRIGGER_HAPPY20" => Some(0x2d3),
        "BTN_TRIGGER_HAPPY21" => Some(0x2d4),
        "BTN_TRIGGER_HAPPY22" => Some(0x2d5),
        "BTN_TRIGGER_HAPPY23" => Some(0x2d6),
        "BTN_TRIGGER_HAPPY24" => Some(0x2d7),
        "BTN_TRIGGER_HAPPY25" => Some(0x2d8),
        "BTN_TRIGGER_HAPPY26" => Some(0x2d9),
        "BTN_TRIGGER_HAPPY27" => Some(0x2da),
        "BTN_TRIGGER_HAPPY28" => Some(0x2db),
        "BTN_TRIGGER_HAPPY29" => Some(0x2dc),
        "BTN_TRIGGER_HAPPY30" => Some(0x2dd),
        "BTN_TRIGGER_HAPPY31" => Some(0x2de),
        "BTN_TRIGGER_HAPPY32" => Some(0x2df),
        "BTN_TRIGGER_HAPPY33" => Some(0x2e0),
        "BTN_TRIGGER_HAPPY34" => Some(0x2e1),
        "BTN_TRIGGER_HAPPY35" => Some(0x2e2),
        "BTN_TRIGGER_HAPPY36" => Some(0x2e3),
        "BTN_TRIGGER_HAPPY37" => Some(0x2e4),
        "BTN_TRIGGER_HAPPY38" => Some(0x2e5),
        "BTN_TRIGGER_HAPPY39" => Some(0x2e6),
        "BTN_TRIGGER_HAPPY40" => Some(0x2e7),

        /* We avoid low common keys in module aliases so they don't get huge. */
        "KEY_MIN_INTERESTING" => Some(KEY_MUTE),
        "KEY_MAX" => Some(0x2ff),
        "KEY_CNT" => Some(KEY_MAX + 1),

        /*
         * Relative axes
         */
        "REL_X" => Some(0x00),
        "REL_Y" => Some(0x01),
        "REL_Z" => Some(0x02),
        "REL_RX" => Some(0x03),
        "REL_RY" => Some(0x04),
        "REL_RZ" => Some(0x05),
        "REL_HWHEEL" => Some(12), // high resolution wheel
        "REL_DIAL" => Some(0x07),
        "REL_WHEEL" => Some(11), // high resolution wheel
        "REL_MISC" => Some(0x09),
        "REL_MAX" => Some(0x0f),
        "REL_CNT" => Some(REL_MAX + 1),

        /*
         * Absolute axes
         */
        "ABS_X" => Some(0x00),
        "ABS_Y" => Some(0x01),
        "ABS_Z" => Some(0x02),
        "ABS_RX" => Some(0x03),
        "ABS_RY" => Some(0x04),
        "ABS_RZ" => Some(0x05),
        "ABS_THROTTLE" => Some(0x06),
        "ABS_RUDDER" => Some(0x07),
        "ABS_WHEEL" => Some(0x08),
        "ABS_GAS" => Some(0x09),
        "ABS_BRAKE" => Some(0x0a),
        "ABS_HAT0X" => Some(0x10),
        "ABS_HAT0Y" => Some(0x11),
        "ABS_HAT1X" => Some(0x12),
        "ABS_HAT1Y" => Some(0x13),
        "ABS_HAT2X" => Some(0x14),
        "ABS_HAT2Y" => Some(0x15),
        "ABS_HAT3X" => Some(0x16),
        "ABS_HAT3Y" => Some(0x17),
        "ABS_PRESSURE" => Some(0x18),
        "ABS_DISTANCE" => Some(0x19),
        "ABS_TILT_X" => Some(0x1a),
        "ABS_TILT_Y" => Some(0x1b),
        "ABS_TOOL_WIDTH" => Some(0x1c),

        "ABS_VOLUME" => Some(0x20),

        "ABS_MISC" => Some(0x28),

        "ABS_MT_SLOT" => Some(0x2f),        /* MT slot being modified */
        "ABS_MT_TOUCH_MAJOR" => Some(0x30), /* Major axis of touching ellipse */
        "ABS_MT_TOUCH_MINOR" => Some(0x31), /* Minor axis " => Some(omit if circular */
        "ABS_MT_WIDTH_MAJOR" => Some(0x32), /* Major axis of approaching ellipse */
        "ABS_MT_WIDTH_MINOR" => Some(0x33), /* Minor axis " => Some(omit if circular */
        "ABS_MT_ORIENTATION" => Some(0x34), /* Ellipse orientation */
        "ABS_MT_POSITION_X" => Some(0x35),  /* Center X touch position */
        "ABS_MT_POSITION_Y" => Some(0x36),  /* Center Y touch position */
        "ABS_MT_TOOL_TYPE" => Some(0x37),   /* Type of touching device */
        "ABS_MT_BLOB_ID" => Some(0x38),     /* Group a set of packets as a blob */
        "ABS_MT_TRACKING_ID" => Some(0x39), /* Unique ID of initiated contact */
        "ABS_MT_PRESSURE" => Some(0x3a),    /* Pressure on contact area */
        "ABS_MT_DISTANCE" => Some(0x3b),    /* Contact hover distance */
        "ABS_MT_TOOL_X" => Some(0x3c),      /* Center X tool position */
        "ABS_MT_TOOL_Y" => Some(0x3d),      /* Center Y tool position */

        "ABS_MAX" => Some(0x3f),
        "ABS_CNT" => Some(ABS_MAX + 1),

        /*
         * Switch events
         */
        "SW_LID" => Some(0x00),                  /* set = lid shut */
        "SW_TABLET_MODE" => Some(0x01),          /* set = tablet mode */
        "SW_HEADPHONE_INSERT" => Some(0x02),     /* set = inserted */
        "SW_RFKILL_ALL" => Some(0x03), /* rfkill master switch), type "any" set = radio enabled */
        "SW_RADIO" => Some(SW_RFKILL_ALL), /* deprecated */
        "SW_MICROPHONE_INSERT" => Some(0x04), /* set = inserted */
        "SW_DOCK" => Some(0x05),       /* set = plugged into dock */
        "SW_LINEOUT_INSERT" => Some(0x06), /* set = inserted */
        "SW_JACK_PHYSICAL_INSERT" => Some(0x07), /* set = mechanical switch set */
        "SW_VIDEOOUT_INSERT" => Some(0x08), /* set = inserted */
        "SW_CAMERA_LENS_COVER" => Some(0x09), /* set = lens covered */
        "SW_KEYPAD_SLIDE" => Some(0x0a), /* set = keypad slide out */
        "SW_FRONT_PROXIMITY" => Some(0x0b), /* set = front proximity sensor active */
        "SW_ROTATE_LOCK" => Some(0x0c), /* set = rotate locked/disabled */
        "SW_LINEIN_INSERT" => Some(0x0d), /* set = inserted */
        "SW_MUTE_DEVICE" => Some(0x0e), /* set = device disabled */
        "SW_MAX" => Some(0x0f),
        "SW_CNT" => Some(SW_MAX + 1),

        /*
         * Misc events
         */
        "MSC_SERIAL" => Some(0x00),
        "MSC_PULSELED" => Some(0x01),
        "MSC_GESTURE" => Some(0x02),
        "MSC_RAW" => Some(0x03),
        "MSC_SCAN" => Some(0x04),
        "MSC_TIMESTAMP" => Some(0x05),
        "MSC_MAX" => Some(0x07),
        "MSC_CNT" => Some(MSC_MAX + 1),

        /*
         * LEDs
         */
        "LED_NUML" => Some(0x00),
        "LED_CAPSL" => Some(0x01),
        "LED_SCROLLL" => Some(0x02),
        "LED_COMPOSE" => Some(0x03),
        "LED_KANA" => Some(0x04),
        "LED_SLEEP" => Some(0x05),
        "LED_SUSPEND" => Some(0x06),
        "LED_MUTE" => Some(0x07),
        "LED_MISC" => Some(0x08),
        "LED_MAIL" => Some(0x09),
        "LED_CHARGING" => Some(0x0a),
        "LED_MAX" => Some(0x0f),
        "LED_CNT" => Some(LED_MAX + 1),

        /*
         * Autorepeat values
         */
        "REP_DELAY" => Some(0x00),
        "REP_PERIOD" => Some(0x01),
        "REP_MAX" => Some(0x01),
        "REP_CNT" => Some(REP_MAX + 1),

        /*
         * Sounds
         */
        "SND_CLICK" => Some(0x00),
        "SND_BELL" => Some(0x01),
        "SND_TONE" => Some(0x02),
        "SND_MAX" => Some(0x07),
        "SND_CNT" => Some(SND_MAX + 1),
        str => str.parse().ok(),
    }
}
