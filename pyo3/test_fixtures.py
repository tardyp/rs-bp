# this test parse all Android.bp in the test_db.tar.xz
import tarfile
from android_bp import BluePrint

def test_fixtures():
    with tarfile.open('../src/test_db.tar.xz', 'r') as tar:
        for member in tar.getmembers():
            # skip links (KeyError: "linkname './build/soong/root.bp' not found")
            if member.name.endswith('Android.bp') and not member.issym():
                with tar.extractfile(member) as f:
                    bp = BluePrint.parse(f.read().decode('utf-8'))
                    assert bp is not None

# test the example from the README
def test_from_file():
    bp = BluePrint.from_file('../fixtures/example.bp')
    assert bp is not None

    # modules are accessible as a python list
    for m in bp.modules:
        if m.__type__ == "rust_binary":
            # module properties can be accessed directly as python attributes
            assert m.name is not None

    # or iter them by type
    for m in bp.modules_by_type("rust_host_test"):
        # or via __dict__
        assert m.__dict__["name"] is not None

        # for convenience, unknown properties return None (not an AttributeError)
        assert m.unknown_attribute is None

        # map properties are accessible as python dicts
        assert m.test_options['unit_test'] is not None