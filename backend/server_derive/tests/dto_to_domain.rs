use server_derive::EnumDtoToDomain;

#[test]
fn dto_to_domain_works() {
    #[derive(EnumDtoToDomain)]
    #[dto_to_domain(target = Test)]
    enum TestDto {
        A,
    }

    #[derive(Debug, PartialEq, Eq)]
    enum Test {
        A,
    }

    let a_dto = TestDto::A;
    let a: Test = a_dto.into();

    assert_eq!(Test::A, a);
}
