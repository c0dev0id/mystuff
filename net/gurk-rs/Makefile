COMMENT =	Signal Messenger client for terminal

V =		0.2.5
GH_ACCOUNT =	boxdot
GH_PROJECT =	gurk-rs
GH_TAGNAME =	v${V}

CATEGORIES =	net

MAINTAINER =	Stefan Hagen <sh+ports@codevoid.de>

# LICENSE
PERMIT_PACKAGE =	Yes

# as devel/cargo MODULES adds DISTFILES, GH_* didn't
DISTFILES +=	${DISTNAME}${EXTRACT_SUFX}

MODULES =	devel/cargo

.include "crates.inc"

WANTLIB =	c c++abi pthread

#LIB_DEPENDS =	
#RUN_DEPENDS =	
#BUILD_DEPENDS =	

#TEST_DEPENDS =	

CONFIGURE_STYLE =	cargo

#MAKE_FLAGS =	

#NO_TEST =	Yes
#TEST_TARGET =	

pre-configure:
	cat ${FILESDIR}/config >> ${WRKSRC}/.cargo/config.toml; \
	cp -rf ${FILESDIR}/vendor ${WRKSRC}/

.include <bsd.port.mk>
