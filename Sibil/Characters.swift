//
//  Characters.swift
//  Sibil
//
//  Created by Eryn Wells on 12/19/16.
//  Copyright © 2016 Eryn Wells. All rights reserved.
//

import Foundation


extension CharacterSet {
    static let asciiLetters: CharacterSet = {
        return CharacterSet(charactersIn: "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ")
    }()

    static let asciiDigits: CharacterSet = {
        return CharacterSet(charactersIn: "1234567890")
    }()

    static let identifierInitials: CharacterSet = {
        let letters = CharacterSet.asciiLetters
        let extras = CharacterSet(charactersIn: "!$%&*/:<=>?~_^")
        let initials = letters.union(extras)
        return initials
    }()

    static let identifierSubsequents: CharacterSet = {
        let initials = CharacterSet.identifierInitials
        let digits = CharacterSet.asciiDigits
        let extras = CharacterSet(charactersIn: ".+-")
        let subsequents = initials.union(digits).union(extras)
        return subsequents
    }()

    func contains(_ c: Character) -> Bool {
        let cSet = CharacterSet(charactersIn: String(c))
        let containsC = isSuperset(of: cSet)
        return containsC
    }
}


extension Character {
    var isLeftParen: Bool {
        return self == "("
    }

    var isRightParen: Bool {
        return self == ")"
    }

    var isIdentifierInitial: Bool {
        return CharacterSet.identifierInitials.contains(self)
    }

    var isIdentifierSubsequent: Bool {
        return CharacterSet.identifierSubsequents.contains(self)
    }
}
