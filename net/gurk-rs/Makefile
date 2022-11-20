COMMENT =	Signal Messenger client for terminal

GH_ACCOUNT =	boxdot
GH_PROJECT =	gurk-rs
GH_TAGNAME =	v0.3.0

CATEGORIES =	net

MAINTAINER =	Stefan Hagen <sh+ports@codevoid.de>

# AGPL 3.0
PERMIT_PACKAGE =	Yes

DISTFILES =	${DISTNAME}{master}${EXTRACT_SUFX}

# vendor files (see $FILESDIR/config)
MASTER_SITES0 =	https://codevoid.de/h/
DISTFILES +=	${DISTNAME}-vendorfiles${EXTRACT_SUFX}:0

MODULES =	devel/cargo

#MODCARGO_NO_DEFAULT_FEATURES =	Yes

WANTLIB =	c c++abi pthread

CONFIGURE_STYLE =	cargo

post-extract:
	mv ${WRKDIR}/vendor ${WRKSRC}/vendor;

pre-configure:
	cat ${FILESDIR}/config >> ${WRKSRC}/.cargo/config

.include "crates.inc"

.include <bsd.port.mk>
