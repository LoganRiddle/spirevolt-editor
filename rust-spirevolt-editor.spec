# Generated by rust2rpm 24
%bcond_without check

%global crate spirevolt-editor

Name:           rust-spirevolt-editor
Version:        0.1.9
Release:        %autorelease
Summary:        # FIXME

License:        None
FIXME: No license information in crate metadata.
URL:            https://crates.io/crates/spirevolt-editor
Source:         %{crates_source}

BuildRequires:  rust-packaging >= 21

%global _description %{expand:
%{summary}.}

%description %{_description}

%package     -n %{crate}
Summary:        %{summary}

%description -n %{crate} %{_description}

%files       -n %{crate}
# FIXME: no license files detected
%{_bindir}/spirevolt-editor

%prep
%autosetup -n %{crate}-%{version_no_tilde} -p1
%cargo_prep

%generate_buildrequires
%cargo_generate_buildrequires

%build
%cargo_build

%install
%cargo_install

%if %{with check}
%check
%cargo_test
%endif

%changelog
%autochangelog
