/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ContentTagType {
    EmailAddress,
    PhoneNumber,
    PhysicalAddress,
    ContactInfo,
    Health,
    Fitness,
    PaymentInfo,
    CreditInfo,
    FinancialInfo,
    PreciseLocation,
    CoarseLocation,
    SensitiveInfo,
    Contacts,
    Messages,
    PhotoVideo,
    Audio,
    GameplayContent,
    CustomerSupport,
    UserContent,
    BrowsingHistory,
    SearchHistory,
    UserId,
    DeviceId,
    PurchaseHistory,
    ProductInteraction,
    AdvertisingData,
    UsageData,
    CrashData,
    PerformanceData,
    DiagnosticData,
    Custom,
}
