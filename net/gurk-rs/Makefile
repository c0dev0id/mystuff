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

WANTLIB =	${MODCARGO_WANTLIB}

BUILD_DEPENDS =	devel/protobuf

CONFIGURE_STYLE =	cargo

post-configure:
	cat ${FILESDIR}/config >> ${WRKDIR}/.cargo/config
	sed -i.bak 's/opt-level = 2/opt-level = 0/g' ${WRKDIR}/.cargo/config

.include "crates.inc"

.include <bsd.port.mk>
