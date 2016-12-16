//
//  Lexer.swift
//  Sibil
//
//  Created by Eryn Wells on 12/16/16.
//  Copyright © 2016 Eryn Wells. All rights reserved.
//

import Foundation


struct Token {
    enum Kind {
        case LeftParen
        case RightParen
    }
    
    let kind: Kind
    let value: String
}


class Lexer: IteratorProtocol {
    typealias Element = Token

    let input: String

    private var index: String.Index

    init(input: String) {
        self.input = input
        self.index = input.startIndex
    }

    // MARK: IteratorProtocol

    func next() -> Token? {
        var token: Token?
        while token == nil {
            let c = input[index]
            switch c {
            case "(":
                token = Token(kind: .LeftParen, value: String(c))
            case ")":
                token = Token(kind: .RightParen, value: String(c))
            default:
                break
            }
        }
        return token
    }
}
